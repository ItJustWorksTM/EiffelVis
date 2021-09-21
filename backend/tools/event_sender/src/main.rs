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
    #[structopt(default_value = "1", short, long)]
    count: usize,

    #[structopt(default_value = "amqp://127.0.0.1:5672/%2f", short, long)]
    url: String,

    #[structopt(default_value = "amq.fanout", short, long)]
    exchange: String,

    #[structopt(short, long)]
    routing_key: String,

    #[structopt(long)]
    seed: Option<usize>,
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

    let thing = EventGenerator::new(
        cli.seed.unwrap_or_else(|| thread_rng().gen::<usize>()),
        100,
        100,
        EventSet::build()
            .add_link(Link::new("Link0", true))
            .add_link(Link::new("Link1", true))
            .add_event(
                Event::new("Event", "1.0.0")
                    .with_link("Link0")
                    .with_link("Link1"),
            )
            .build()
            .expect("This should work"),
    );

    println!("Sending out {} events..", cli.count);

    for ev in thing.iter().take(cli.count) {
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
    }

    println!("Done.");

    Ok(())
}
