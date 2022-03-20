mod domain;
mod utils;

use std::time::Duration;

use clap::{App, Arg};
use log::info;

use rdkafka::config::ClientConfig;
use rdkafka::consumer::stream_consumer::StreamConsumer;
use rdkafka::consumer::Consumer;
use rdkafka::message::{BorrowedMessage, OwnedMessage};
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::util::get_rdkafka_version;
use rdkafka::Message;

use avro_rs::from_value;
use futures::stream::FuturesUnordered;
use rand::Rng;
use schema_registry_converter::async_impl::avro::AvroDecoder;
use schema_registry_converter::async_impl::easy_avro::EasyAvroEncoder;
use schema_registry_converter::async_impl::schema_registry::SrSettings;
use schema_registry_converter::schema_registry_common::SubjectNameStrategy;

use crate::domain::power_event::PowerEvent;
use crate::domain::power_request::PowerRequest;
use crate::utils::prom_utils::setup_prom_and_log;

use crate::utils::sr_utils::get_sr_settings;
use tokio_stream::{self as stream, StreamExt};

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

async fn consume(brokers: &str, input_topic: &str, city_name: &str) {
    let topic = String::from("power-request");

    let sr_settings = get_sr_settings();

    let publish_schema_resutl =
        PowerRequest::publish_schema(&sr_settings, format!("{}-value", topic.clone()))
            .await
            .expect("fail to publish schema");
    let encoder = EasyAvroEncoder::new(sr_settings);
    let primitive_schema_strategy = SubjectNameStrategy::TopicNameStrategy(topic.clone(), false);
    info!("registered Schema :  {:?}", publish_schema_resutl);

    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", "city-simple")
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

    let item_stream = futures::stream::repeat("tick").throttle(Duration::from_secs(10));
    tokio::pin!(item_stream);

    let mut rng = rand::thread_rng();

    loop {
        let _ = item_stream.next().await;

        let mut power_need: u32 = rng.gen_range(0..1000);

        let power_request = PowerRequest {
            origin: String::from(city_name),
            volume: String::from(format! {"{}", power_need.clone()}),
        };

        let bytes = encoder
            .encode_struct(power_request, &primitive_schema_strategy)
            .await
            .expect("can't encode message");

        let record: FutureRecord<String, Vec<u8>> = FutureRecord::to(&topic).payload(&bytes);

        let delivery_status = producer.send(record, Duration::from_secs(0)).await;

        info!(
            "Power request status for {} :  {:?}",
            city_name, delivery_status
        );

        //todo consume power
        // let stream_processor = consumer.stream().take_until()

        info!("Starting event loop");
        // stream_processor.await.expect("stream processing failed");
        info!("Stream processing terminated");
    }
}

#[tokio::main]
async fn main() {
    let matches = App::new("producer example")
        .version(option_env!("CARGO_PKG_VERSION").unwrap_or(""))
        .about("Simple command line producer")
        .arg(
            Arg::new("input-topic")
                .short('i')
                .long("input-topic")
                .help("input line to transformer")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::new("city-name")
                .short('n')
                .long("city-name")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::new("prom-port")
                .short('m')
                .long("prom-port")
                .help("promeheus port")
                .takes_value(true)
                .required(true)
                .default_value("9000"),
        )
        .get_matches();

    let input_topic = matches.value_of("input-topic").unwrap();
    let city_name = matches.value_of("city-name").unwrap();

    let prom_port = matches.value_of("prom-port").unwrap();
    let brokers = "localhost:9092";

    setup_prom_and_log(prom_port);

    info!("input : {}", input_topic);
    info!("city_name : {}", city_name);

    let (version_n, version_s) = get_rdkafka_version();
    info!("rd_kafka_version: 0x{:08x}, {}", version_n, version_s);

    consume(brokers, input_topic, city_name).await;
}
