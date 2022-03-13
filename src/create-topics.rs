mod utils;

use std::time::Duration;

use clap::{value_t, App, Arg};
use log::{info, trace};
use rdkafka::admin::{AdminClient, AdminOptions, NewTopic, TopicReplication};
use rdkafka::client::DefaultClientContext;

use rdkafka::config::ClientConfig;
use rdkafka::consumer::{BaseConsumer, Consumer};

use crate::utils::prom_utils::setup_prom_and_log;

fn create_config() -> ClientConfig {
    let mut config = ClientConfig::new();
    config.set("bootstrap.servers", "0.0.0.0:9092");
    config
}

fn create_admin_client() -> AdminClient<DefaultClientContext> {
    create_config()
        .create()
        .expect("admin client creation failed")
}

#[tokio::main]
async fn main() {

    setup_prom_and_log("9988");

    let admin_client = create_admin_client();
    let opts = AdminOptions::new().operation_timeout(Some(Duration::from_secs(10)));



        // Test both the builder API and the literal construction.
        let lyon_bt =
            NewTopic::new("lyon-bt-v2", 10, TopicReplication::Fixed(2)).set("max.message.bytes", "1234");
        let west_tht =
            NewTopic::new("west-tht-v2", 30, TopicReplication::Fixed(2)).set("max.message.bytes", "1234");
        let nuc_001_ht =
            NewTopic::new("nuc-001-ht-v2", 20, TopicReplication::Fixed(2)).set("max.message.bytes", "1234");
        let power_request =
            NewTopic::new("power-request-v2", 1, TopicReplication::Fixed(3)).set("max.message.bytes", "1234");


        let res = admin_client
            .create_topics(&[lyon_bt, west_tht,nuc_001_ht,power_request], &opts)
            .await
            .expect("topic creation failed");

    info!("creation status : {:?}",res)
}