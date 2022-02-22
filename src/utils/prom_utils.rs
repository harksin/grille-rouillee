use metrics_exporter_prometheus::PrometheusBuilder;
use metrics_util::MetricKindMask;
use std::net::SocketAddr;
use std::time::Duration;

pub fn setup_prom_and_log(prom_port: &str) {
    tracing_subscriber::fmt::init();
    let prom_reporter = PrometheusBuilder::new();

    let prom_listener: SocketAddr = format!("0.0.0.0:{}", prom_port)
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
}
