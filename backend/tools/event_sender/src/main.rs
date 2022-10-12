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

#[derive(Parser)]
#[clap(about = "Generates random events and sends them over ampq")]
struct Cli {
    /// Total amount of events to be sent (note: multiplied with the `burst` option)
    #[clap(default_value = "300", short, long)]
    count: usize,

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

    /// Time in milliseconds to sleep before emitting a new burst of events
    #[clap(default_value = "0", short, long)]
    latency: usize,

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

    println!("Connected to broker.");
    let type_Array = ["Event1","Event2","Event3","Event4","Event5"];
    
    let gen = EventGenerator::new(
        cli.seed.unwrap_or_else(|| thread_rng().gen::<usize>()),
        4,
        8,
        EventSet::build()
            .add_link(Link::new("Link0", true))
            .add_link(Link::new("Link1", true))
            .add_event(
<<<<<<< HEAD
                Event::new(type_Array[rand::thread_rng().gen_range(0..4)] , "1.0.0")
=======
                Event::new(type_array[rand::thread_rng().gen_range(0..4)] , "1.0.0")
>>>>>>> e883938 (backend/event_sender: fix clippy warning)
                    .with_link("Link0")
                    .with_link("Link1"),
            )
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
    fn random_num(from:usize, to:usize) -> usize{
        let mut rng = rand::thread_rng();
        return rng.gen_range(from..to);
    }

    println!(
        "Sending out a maximum of {} events, over a maximum duration of {} seconds. \nEvents sent at random intervals between 5-220ms. \nProcess will stop at whichever comes first.\n",
        target, ((target*120) / 1000)
    );

    let mut sent = 0;
    // Used for a time reference staring point
    let start = Instant::now();
    // Factor to calculate the total duration from the event count
    let factor = 120;
    // Total duration is = to the count multiplied by 120 (30000 events will be sent over and hour)
    let total_duration = cli.count * factor;
    // Decalred as mut in order to allow the value to change
    let mut duration = start.elapsed();

        for _ in 0..(target) {
            // Loop until the elapsed time has reached the calculated total duration or event count has been reached
            while duration.as_millis() < total_duration.try_into().unwrap() && sent < cli.count{
                // Generate a random delay between 5 and 220ms
                let pause_duration = Duration::from_millis(random_num(5, 215) as u64);
                //println!("Pause duration: {:?}", pause_duration); 

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
    println!("Done! Total events sent: {}, total duration: {:?}", sent, duration);

    Ok(())
}