use std::{
    fs::File,
    io::{BufRead, BufReader},
    time::Duration,
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
    #[clap(default_value = "1", short, long)]
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
    let type_array = ["Event1", "Event2", "Event3", "Event4", "Event5"];

    let gen = EventGenerator::new(
        cli.seed.unwrap_or_else(|| thread_rng().gen::<usize>()),
        4,
        8,
        EventSet::build()
            .add_link(Link::new("Link0", true))
            .add_link(Link::new("Link1", true))
            .add_event(
                Event::new(type_array[rand::thread_rng().gen_range(0..4)], "1.0.0")
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
    let sleep_duration = Duration::from_millis(cli.latency as u64);

    println!(
        "Sending out ~{} events, {} events every {}ms interval",
        target, cli.burst, cli.latency
    );

    let mut sent = 0;
    for _ in 0..(target) {
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
        if sleep_duration.as_millis() > 0 {
            tokio::time::sleep(sleep_duration).await;
        }
    }

    println!("Done, sent {} events", sent);

    Ok(())
}
