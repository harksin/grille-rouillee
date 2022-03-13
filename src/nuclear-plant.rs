mod domain;
mod utils;

use avro_rs::from_value;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;

use clap::{App, Arg};
use log::info;

use metrics::increment_gauge;

use rdkafka::config::ClientConfig;
use rdkafka::consumer::stream_consumer::StreamConsumer;
use rdkafka::consumer::Consumer;
use rdkafka::message::{BorrowedMessage, OwnedMessage};
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::util::get_rdkafka_version;
use rdkafka::Message;

use chrono::prelude::*;
use futures::TryStreamExt;
use schema_registry_converter::async_impl::avro::AvroDecoder;

use tokio_stream::{self as stream, StreamExt};

use crate::domain::power_event::PowerEvent;
use crate::domain::power_request::PowerRequest;
use crate::utils::prom_utils::setup_prom_and_log;
use crate::utils::sr_utils::get_sr_settings;
use schema_registry_converter::async_impl::easy_avro::EasyAvroEncoder;
use schema_registry_converter::async_impl::schema_registry::SrSettings;
use schema_registry_converter::schema_registry_common::SubjectNameStrategy;
use tokio::join;

async fn command_listener(
    brokers: &str,
    power_plant_name: &str,
    current_production: Arc<AtomicUsize>,
) {
    let sr_settings = get_sr_settings();

    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", "transfo-simple")
        .set("bootstrap.servers", brokers)
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "true")
        .create()
        .expect("Consumer creation failed");

    let production_command_topic = "power-request";

    consumer
        .subscribe(&[production_command_topic])
        .expect("Can't subscribe to specified topic");

    let stream_processor = consumer.stream().try_for_each(|borrowed_message| {
        let mut decoder = AvroDecoder::new(sr_settings.clone());
        let _current_production = current_production.clone();
        async move {
            let owned_message: OwnedMessage = borrowed_message.detach();
            let raw = decoder
                .decode(owned_message.payload())
                .await
                .expect("can't parse avro payload");
            let pwr: PowerRequest =
                from_value::<PowerRequest>(&raw.value).expect("can't map avro payload");

            _current_production
                .clone()
                .store(pwr.volume.parse().unwrap(), Ordering::SeqCst);

            info!(
                "updated production to {} Mw",
                _current_production.clone().load(Ordering::SeqCst)
            );
            Ok(())
        }
    });

    stream_processor.await.expect("stream processing failed");
}

async fn producer_loop(
    brokers: &str,
    power_plant_name: &str,
    current_production: Arc<AtomicUsize>,
) {
    let topic_name = format!("{}-ht", power_plant_name);
    info!("output : {}", topic_name);

    let sr_settings = get_sr_settings();
    let publish_schema_resutlt =
        PowerEvent::publish_schema(&sr_settings, format!("{}-value", topic_name))
            .await
            .expect("fail to publish schema");
    info!("registered Schema :  {:?}", publish_schema_resutlt);
    let encoder = EasyAvroEncoder::new(sr_settings.clone());

    let primitive_schema_strategy =
        SubjectNameStrategy::TopicNameStrategy(topic_name.clone(), false);

    let producer: &FutureProducer = &ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Producer creation error");

    let mut interval_timer = tokio::time::interval(chrono::Duration::seconds(2).to_std().unwrap());
    loop {
        // Wait for the next interval tick
        interval_timer.tick().await;

        let power_event = PowerEvent {
            origin: String::from(power_plant_name),
            volume: current_production.load(Ordering::SeqCst).to_string(),
            ts: format!("{:?}", Utc::now()),
            power_type: String::from("HT"),
        };

        let bytes = encoder
            .encode_struct(power_event.clone(), &primitive_schema_strategy)
            .await
            .expect("can't encode message");

        let delivery_status = producer
            .send(
                FutureRecord::to(&topic_name)
                    .payload(&bytes)
                    .key(&format!("Key {:?}", power_plant_name)),
                Duration::from_secs(0),
            )
            .await;

        info!(
            "produced {} Mw, status:  {:?}",
            power_event.volume, delivery_status
        );

        increment_gauge!("message_sent", 1.0);
    }
}

#[tokio::main]
async fn main() {
    let matches = App::new("producer example")
        .version(option_env!("CARGO_PKG_VERSION").unwrap_or(""))
        .about("Simple command line producer")
        .arg(
            Arg::with_name("brokers")
                .short("b")
                .long("brokers")
                .help("Broker list in kafka format")
                .takes_value(true)
                .default_value("localhost:9092"),
        )
        .arg(
            Arg::with_name("log-conf")
                .long("log-conf")
                .help("Configure the logging format (example: 'rdkafka=trace')")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("power-plant-name")
                .short("n")
                .long("power-plant-name")
                .help("just a name to identify the power plan")
                .takes_value(true)
                .required(true)
                .default_value("nuc-001"),
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

    let name = matches.value_of("power-plant-name").unwrap();
    let brokers = matches.value_of("brokers").unwrap();
    let prom_port = matches.value_of("prom-port").unwrap();

    info!("plant name   : {}", name);

    setup_prom_and_log(prom_port);

    let (version_n, version_s) = get_rdkafka_version();
    info!("rd_kafka_version: 0x{:08x}, {}", version_n, version_s);

    let current_production: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(1));

    let p_loop = producer_loop(brokers, name, current_production.clone());
    let c_loop = command_listener(brokers, name, current_production.clone());

    join!(p_loop, c_loop);
}
