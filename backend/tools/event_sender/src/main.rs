use std::time::Duration;

use eiffelvis_gen::{
    event_set::{Event, EventSet, Link},
    generator::EventGenerator,
};
use lapin::{options::*, BasicProperties, Connection, ConnectionProperties};

use rand::{thread_rng, Rng};
use structopt::StructOpt;
use tokio_amqp::LapinTokioExt;

#[derive(StructOpt)]
#[structopt(
    name = "Eiffel Event Sender",
    about = "Generates random events and sends them over ampq"
)]
struct Cli {
    /// Total amount of events to be sent (note: multiplied with the `burst` option)
    #[structopt(default_value = "1", short, long)]
    count: usize,

    /// URL to amqp server
    #[structopt(default_value = "amqp://127.0.0.1:5672/%2f", short, long)]
    url: String,

    /// Ampq exchange to send events to
    #[structopt(default_value = "amq.fanout", short, long)]
    exchange: String,

    /// Routing key used for ampq connections
    #[structopt(short, long)]
    routing_key: String,

    /// Random seed used to create event data
    #[structopt(long)]
    seed: Option<usize>,

    /// Time in milliseconds to sleep before emitting a new burst of events
    #[structopt(default_value = "0", short, long)]
    latency: usize,

    /// Amount of events to send before introducing another delay (defined with the latency option)
    #[structopt(default_value = "1", short, long)]
    burst: usize,
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

    let conn = Connection::connect(addr, ConnectionProperties::default().with_tokio()).await?;

    let channel_a = conn.create_channel().await?;

    println!("Connected to broker.");

    let event_set = EventSet::build()
        .add_link(Link::new("Link0", true).with_target("Unique"))
        .add_link(Link::new("Link1", true).with_target("NotUnique"))
        .add_event(Event::new("Unique", "1.0.0"))
        .add_event(
            Event::new("NotUnique", "1.0.0")
                .with_link("Link1")
                .with_link("Link0"),
        )
        .add_event(Event::new("EventLOL", "1.0.0").with_req_link("Link0"))
        .build()
        .expect("This should work");

    println!("{:#?}", event_set);

    let gen = EventGenerator::new(
        cli.seed.unwrap_or_else(|| thread_rng().gen::<usize>()),
        4,
        8,
        event_set,
    );

    let target = cli.count * cli.burst;
    let sleep_duration = Duration::from_millis(cli.latency as u64);

    println!(
        "Sending out ~{} events, 0..={} events every {}ms interval",
        target, cli.burst, cli.latency
    );

    let mut iter = gen.iter();
    let mut sent = 0;
    while sent < target {
        for ev in (&mut iter).take(thread_rng().gen_range(0..cli.burst + 1)) {
            let _ = channel_a
                .basic_publish(
                    cli.exchange.as_str(),
                    cli.routing_key.as_str(),
                    BasicPublishOptions::default(),
                    ev,
                    BasicProperties::default(),
                )
                .await?
                .await?;
            sent += 1;
        }
        if sleep_duration.as_millis() > 0 {
            tokio::time::sleep(sleep_duration).await;
        }
    }

    println!("Done, sent {} events", sent);

    Ok(())
}
