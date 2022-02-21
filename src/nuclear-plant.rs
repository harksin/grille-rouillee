mod domain;

use std::net::SocketAddr;
use std::time::Duration;

use clap::{App, Arg};
use log::info;

use metrics::increment_gauge;
use metrics_exporter_prometheus::PrometheusBuilder;
use metrics_util::MetricKindMask;

use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::util::get_rdkafka_version;

use chrono::prelude::*;

use tokio_stream::{self as stream, StreamExt};

use schema_registry_converter::async_impl::easy_avro::EasyAvroEncoder;
use schema_registry_converter::async_impl::schema_registry::SrSettings;
use schema_registry_converter::schema_registry_common::SubjectNameStrategy;
use crate::domain::power_event::PowerEvent;
// use serde_json::Value::String;

async fn produce(brokers: &str, power_plant_name: &str) {

    let topic_name = format!("{}-ht", power_plant_name);
    let sr_url = String::from("http://localhost:8081");
    let sr_settings = SrSettings::new(sr_url);
    let publish_schema_resutl = PowerEvent::publish_schema(&sr_settings,format!("{}-value", topic_name)).await.expect("fail to publish schema");
    info!(
            "registered Schema :  {:?}",
            publish_schema_resutl
        );
    let encoder = EasyAvroEncoder::new(sr_settings);


    let primitive_schema_strategy = SubjectNameStrategy::TopicNameStrategy(
        topic_name.clone(),
        false,
    );




    let producer: &FutureProducer = &ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Producer creation error");

    let item_stream = futures::stream::repeat("tick").throttle(Duration::from_millis(100));
    tokio::pin!(item_stream);

    loop {
        let _ = item_stream.next().await;
        // let message = json!({
        //     "origin": power_plant_name,
        //     "volume": "100Mw",
        //     "ts": &format!("{:?}",Utc::now()),
        //     "power_type": "HT"
        // });

        let power_event = PowerEvent{
            origin: String::from(power_plant_name),
            volume: String::from("100"),
            ts: format!("{:?}",Utc::now()),
            power_type: String::from("HT"),
        };

        let bytes = encoder
            .encode_struct(power_event, &primitive_schema_strategy)
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

        increment_gauge!("message_sent", 1.0);
        info!(
            "Delivery status for plant {} :  {:?}",
            power_plant_name, delivery_status
        );
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

    let name = matches.value_of("power-plant-name").unwrap();
    let brokers = matches.value_of("brokers").unwrap();

    produce(brokers, name).await;
}
