mod domain;
mod utils;

use std::net::SocketAddr;
use std::time::Duration;

use clap::{App, Arg};
use log::info;

use metrics::{increment_counter, increment_gauge};
use metrics_exporter_prometheus::PrometheusBuilder;
use metrics_util::MetricKindMask;

use rdkafka::config::ClientConfig;
use rdkafka::consumer::stream_consumer::StreamConsumer;
use rdkafka::consumer::Consumer;
use rdkafka::message::{BorrowedMessage, OwnedMessage};
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::util::get_rdkafka_version;
use rdkafka::Message;

use avro_rs::from_value;
use futures::stream::FuturesUnordered;
use futures::TryStreamExt;
use schema_registry_converter::async_impl::avro::AvroDecoder;
use schema_registry_converter::async_impl::easy_avro::EasyAvroEncoder;
use schema_registry_converter::async_impl::schema_registry::SrSettings;
use schema_registry_converter::schema_registry_common::SubjectNameStrategy;

// use serde::{Deserialize, Serialize};
// use serde_json::json;
// use avro_rs::{Codec, Reader, Schema, Writer, from_value, types::Record, Error};
use crate::domain::power_event::PowerEvent;
use crate::utils::prom_utils::setup_prom_and_log;

async fn alter_current_type<'a>(
    msg: OwnedMessage,
    sr_settings: SrSettings,
    output_power: String,
    primitive_schema_strategy: SubjectNameStrategy,
) -> Vec<u8> {
    info!("Starting expensive computation on message {}", msg.offset());
    info!(
        "Expensive computation completed on message {}",
        msg.offset()
    );

    // let mut pw: PowerEvent = serde_json::from_slice(msg.payload().unwrap()).unwrap();
    let mut decoder = AvroDecoder::new(sr_settings.clone());
    let encoder = EasyAvroEncoder::new(sr_settings);

    let raw = decoder
        .decode(msg.payload())
        .await
        .expect("can't parse avro payload");
    let mut pw: PowerEvent = from_value::<PowerEvent>(&raw.value).expect("can't map avro payload");

    pw.power_type = output_power;

    // serde_json::to_string(&pw).unwrap()

    encoder
        .encode_struct(pw, &primitive_schema_strategy)
        .await
        .expect("can't encode message")
}

async fn transform(brokers: &str, input_topic: &str, output_topic: &str, output_power: &str) {
    let sr_url = String::from("http://localhost:8081");
    let sr_settings = SrSettings::new(sr_url);
    let publish_schema_resutl =
        PowerEvent::publish_schema(&sr_settings, format!("{}-value", output_topic))
            .await
            .expect("fail to publish schema");
    info!("registered Schema :  {:?}", publish_schema_resutl);

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

        let sr_setting_local = sr_settings.clone();

        async move {
            increment_gauge!("power_event_converted", 1.0);
            let owned_message: OwnedMessage = borrowed_message.detach();

            let primitive_schema_strategy: SubjectNameStrategy =
                SubjectNameStrategy::TopicNameStrategy(output_topic.clone(), false);

            let computation_result = alter_current_type(
                owned_message,
                sr_setting_local,
                String::from(output_power),
                primitive_schema_strategy,
            )
            .await;

            let record: FutureRecord<String, Vec<u8>> =
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
                .help("input line to transformer")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("output-topic")
                .short("o")
                .long("output-topic")
                .help("output line from transformer")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("output-power")
                .short("p")
                .long("output-power")
                .help("output power [BT/HT/THT]")
                .takes_value(true)
                .required(true)
                .default_value("N/A"),
        )
        .arg(
            Arg::with_name("prom-port")
                .short("m")
                .long("prom-port")
                .help("promeheus port")
                .takes_value(true)
                .required(true)
                .default_value("9000"),
        )
        .get_matches();

    let input_topic = matches.value_of("input-topic").unwrap();
    let output_topic = matches.value_of("output-topic").unwrap();
    let output_power = matches.value_of("output-power").unwrap();
    let prom_port = matches.value_of("prom-port").unwrap();
    let brokers = "localhost:9092";

    setup_prom_and_log(prom_port);

    info!("input : {}", input_topic);
    info!("output : {}", output_topic);
    info!("output power : {}", output_power);

    let (version_n, version_s) = get_rdkafka_version();
    info!("rd_kafka_version: 0x{:08x}, {}", version_n, version_s);

    transform(brokers, input_topic, output_topic, output_power).await;
}
