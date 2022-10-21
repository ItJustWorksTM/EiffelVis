use std::{
    fs::File,
    io::{BufRead, BufReader},
    time::Duration,
    time::Instant,
};

use eiffelvis_gen::{
    event_set::{Event, EventSet, Link},
    generator::EventGenerator,
};
use lapin::{options::*, BasicProperties, Connection, ConnectionProperties};

use clap::Parser;
use rand::{thread_rng, Rng};

const EVENT_TYPES: [&str; 23] = [
    "ActC", "ActF", "ActS", "ActT", "AnnP", "ArtC", "ArtP", "ArtR", "TCC", "TCF", "TCS", "TCT",
    "TERCC", "TSF", "TSS", "CD", "CLM", "ED", "FCD", "ID", "IV", "SCC", "SCS",
];

#[derive(Parser)]
#[clap(about = "Generates random events and sends them over ampq")]
struct Cli {
    // NOTE: The number of events along with the total duration, min lacency and latency max is to be used for stress testing.
    // When setting these values, keep in mind that the generator process will stop at whichever value is reached first (between the count and total duration).
    // The defaut settings on these variables represent roughly 30,000 events sent per hour.

    // Total amount of events to be sent (note: multiplied with the `burst` option)
    #[clap(default_value = "30000", short, long)]
    count: usize,

    // Total duration to run the event generator
    #[clap(default_value = "3600000", short, long)]
    total_duration: usize,

    /// URL to amqp server
    #[clap(default_value = "amqp://127.0.0.1:5672/%2f", short, long)]
    url: String,

    /// Ampq exchange to send events to
    #[clap(default_value = "amq.fanout", short, long)]
    exchange: String,

    /// Routing key used for ampq connections
    #[clap(short, long)]
    routing_key: String,

    /// Random seed used to create event data
    #[clap(long)]
    seed: Option<usize>,

    /// Starting latency value for the random interval between events
    #[clap(default_value = "5", short, long)]
    min_latency: usize,

    /// Ending latency value for the random interval between events
    #[clap(default_value = "220", short, long)]
    latency_max: usize,

    /// Amount of events to send before introducing another delay (defined with the latency option)
    #[clap(default_value = "1", short, long)]
    burst: usize,

    #[clap(long)]
    replay: Option<String>,
}

fn main() -> anyhow::Result<()> {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?
        .block_on(app())
}

async fn app() -> anyhow::Result<()> {
    let cli = Cli::from_args();
    let addr = cli.url.as_str();

    let conn = Connection::connect(
        addr,
        #[cfg(unix)]
        ConnectionProperties::default()
            .with_executor(tokio_executor_trait::Tokio::current())
            .with_reactor(tokio_reactor_trait::Tokio),
        #[cfg(not(unix))]
        ConnectionProperties::default().with_executor(tokio_executor_trait::Tokio::current()),
    )
    .await?;

    let channel_a = conn.create_channel().await?;

    channel_a
        .queue_declare(
            "hello",
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await?;

    // println!(?queue, "Declared queue");
    println!("Connected to broker.");

    let mut builder = EventSet::build();

    for eventtype in EVENT_TYPES {
        // Initialize and create the event variable
        let mut event = Event::new(eventtype.to_string(), "1.0.0");

        // Create the random number generate for the links
        let _randomrange = rand::thread_rng().gen_range(1..3);

        // Loop and add the links to the event
        for linknumber in 0.._randomrange {
            event = event.with_link(format!("Link{linknumber}"));
        }

        builder = builder.add_event(event);
    }

    let gen = EventGenerator::new(
        cli.seed.unwrap_or_else(|| thread_rng().gen::<usize>()),
        16,
        18,
        builder
            .add_link(Link::new("Link0", true))
            .add_link(Link::new("Link1", true))
            .build()
            .expect("This should work"),
    );

    let mut iter: Box<dyn Iterator<Item = Vec<u8>>> = if let Some(replay_path) = cli.replay.as_ref()
    {
        println!("replaying file \"{}\"", replay_path);
        let file = File::open(replay_path)?;
        let reader = BufReader::new(file);

        Box::new(reader.lines().map(|err| err.unwrap().as_bytes().to_owned()))
    } else {
        Box::new(gen.iter())
    };

    let target = cli.count * cli.burst;

    // Random number generator used to product random intervals
    fn random_num(from: usize, to: usize) -> usize {
        let mut rng = rand::thread_rng();
        rng.gen_range(from..to)
    }

    println!(
        "\nSending out a maximum of {} events, over a maximum duration of {} seconds. \nProcess will stop at whichever comes first. \nEvents sent at random intervals between {}-{}ms. \n",
        target, (cli.total_duration / 1000), cli.min_latency, cli.latency_max
    );

    let mut sent = 0;
    // Used for a time reference staring point
    let start = Instant::now();
    // Decalred as mut in order to allow the value to change
    let mut duration = start.elapsed();

    for _ in 0..(target) {
        // Loop until the elapsed time has reached the calculated total duration or event count has been reached
        while duration.as_millis() < cli.total_duration.try_into().unwrap() && sent < cli.count {
            // Generate a random delay between 5 and 220ms
            let pause_duration =
                Duration::from_millis(random_num(cli.min_latency, cli.latency_max) as u64);

            let mut taken = 0;
            for ev in (&mut iter).take(cli.burst) {
                let _ = channel_a
                    .basic_publish(
                        cli.exchange.as_str(),
                        cli.routing_key.as_str(),
                        BasicPublishOptions::default(),
                        ev.as_slice(),
                        BasicProperties::default(),
                    )
                    .await?
                    .await?;
                taken += 1;
            }
            if cli.burst > taken {
                println!("Exhausted source, stopping early!");
                break;
            }
            sent += taken;

            // Sleep for a randomly selected time
            tokio::time::sleep(pause_duration).await;
            // Stores the amount of time elapsed from the start.
            duration = start.elapsed();
        }
    }
    println!(
        "Done! Total events sent: {}, total duration: {:?}",
        sent, duration
    );

    Ok(())
}
