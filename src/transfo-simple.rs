mod domain;

use std::net::SocketAddr;
use std::time::Duration;

use clap::{App, Arg};
use log::info;

use metrics::{
    increment_counter
};
use metrics_exporter_prometheus::PrometheusBuilder;
use metrics_util::MetricKindMask;

use rdkafka::config::ClientConfig;
use rdkafka::consumer::stream_consumer::StreamConsumer;
use rdkafka::consumer::Consumer;
use rdkafka::message::{BorrowedMessage, OwnedMessage};
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::util::get_rdkafka_version;
use rdkafka::Message;

use futures::stream::FuturesUnordered;
use futures::{TryStreamExt};

// use serde::{Deserialize, Serialize};
// use serde_json::json;
// use avro_rs::{Codec, Reader, Schema, Writer, from_value, types::Record, Error};
use crate::domain::power_event::PowerEvent;

fn alter_current_type<'a>(msg: OwnedMessage) -> String {
    info!("Starting expensive computation on message {}", msg.offset());
    info!(
        "Expensive computation completed on message {}",
        msg.offset()
    );

    let mut pw: PowerEvent = serde_json::from_slice(msg.payload().unwrap()).unwrap();

    pw.power_type = "THT".to_string();

    serde_json::to_string(&pw).unwrap()
}

async fn transform(brokers: &str, input_topic: &str, output_topic: &str, output_power: &str) {
    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", "transfo-simple")
        .set("bootstrap.servers", brokers)
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "true")
        .create()
        .expect("Consumer creation failed");

    consumer
        .subscribe(&[&input_topic])
        .expect("Can't subscribe to specified topic");

    // Create the `FutureProducer` to produce asynchronously.
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Producer creation error");

    // Create the outer pipeline on the message stream.
    let stream_processor = consumer.stream().try_for_each(|borrowed_message| {
        let producer = producer.clone();
        let output_topic = output_topic.to_string();

        async move {
            let owned_message: OwnedMessage = borrowed_message.detach();

            let computation_result = alter_current_type(owned_message);

            let record: FutureRecord<String, String> =
                FutureRecord::to(&output_topic).payload(&computation_result);

            let produce_future = producer.send(record, Duration::from_secs(0));
            match produce_future.await {
                Ok(delivery) => println!("Sent: {:?}", delivery),
                Err((e, _)) => println!("Error: {:?}", e),
            }
            Ok(())
        }
    });

    info!("Starting event loop");
    stream_processor.await.expect("stream processing failed");
    info!("Stream processing terminated");
}

#[tokio::main]
async fn main() {
    let matches = App::new("producer example")
        .version(option_env!("CARGO_PKG_VERSION").unwrap_or(""))
        .about("Simple command line producer")
        .arg(
            Arg::with_name("input-topic")
                .short("i")
                .long("input-topic")
                .help("just a name to identify the power plan")
                .takes_value(true)
                .required(true)
                .default_value("nuc-001-ht"),
        )
        .arg(
            Arg::with_name("output-topic")
                .short("o")
                .long("output-topic")
                .help("just a name to identify the power plan")
                .takes_value(true)
                .required(true)
                .default_value("test-tr"),
        )
        .arg(
            Arg::with_name("output-power")
                .short("p")
                .long("output-power")
                .help("just a name to identify the power plan")
                .takes_value(true)
                .required(true)
                .default_value("N/A"),
        )
        .get_matches();

    tracing_subscriber::fmt::init();

    let prom_reporter = PrometheusBuilder::new();

    let prom_listener: SocketAddr = "0.0.0.0:9002"
        .parse()
        .expect("not well formated prometheus endpoint");

    prom_reporter
        .idle_timeout(
            MetricKindMask::COUNTER | MetricKindMask::HISTOGRAM,
            Some(Duration::from_secs(10)),
        )
        .with_http_listener(prom_listener)
        .install()
        .expect("failed to install Prometheus recorder");

    let (version_n, version_s) = get_rdkafka_version();
    info!("rd_kafka_version: 0x{:08x}, {}", version_n, version_s);

    let input_topic = matches.value_of("input-topic").unwrap();
    let output_topic = matches.value_of("output-topic").unwrap();
    let output_power = matches.value_of("output-power").unwrap();
    let brokers = "localhost:9092";

    transform(brokers, input_topic, output_topic, output_power).await;
}
