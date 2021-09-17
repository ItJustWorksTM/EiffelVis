use lapin::{options::*, BasicProperties, Connection, ConnectionProperties};

use structopt::StructOpt;
use tokio_amqp::LapinTokioExt;

use eiffelvis_gen::event_type;
use eiffelvis_gen::{link_type, random::EventChainBlueprint};

link_type!(TestLink, true, true, TestEventNice);
event_type!(TestEvent, "1.0.0", TestLinkNice);

link_type!(TestLinkNice, true, true, TestEvent);
event_type!(TestEventNice, "2.0.0", TestLink);

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

    #[structopt(default_value = "0", long)]
    seed: usize,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::from_args();
    let addr = cli.url.as_str();

    let conn = Connection::connect(addr, ConnectionProperties::default().with_tokio()).await?;

    let channel_a = conn.create_channel().await?;

    println!("Connected to broker.");

    let mut thing = EventChainBlueprint::new(0..5, usize::MAX, cli.seed)
        .with(TestEvent)
        .with(TestEventNice)
        .iter();

    println!("Sending out {} events..", cli.count);

    for _ in 0..cli.count {
        let _ = channel_a
            .basic_publish(
                "amq.fanout",
                "hello",
                BasicPublishOptions::default(),
                thing.next().unwrap(),
                BasicProperties::default(),
            )
            .await?
            .await?;
    }

    println!("Done.");

    Ok(())
}
