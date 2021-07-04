use std::{
    sync::{
        mpsc::{self, SyncSender},
        Mutex,
    },
    thread::sleep,
    time::{Duration, Instant},
};

use armaforces_mods::prometheus::{self, MetricsFetcher};
use rand::Rng;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref FETCH: Mutex<bool> = Mutex::new(false);
    static ref METRICS_SENDER: Mutex<Option<SyncSender<prometheus::Metrics>>> = Mutex::new(None);
}

fn export_metrics() {
    println!("requesting metrics");

    std::thread::spawn(|| {
        let mut rng = rand::thread_rng();

        sleep(Duration::from_secs(rng.gen_range(1..4)));
        send_metrics();
    });
}

fn send_metrics() {
    println!("sending metrics");

    let lock = METRICS_SENDER.lock().unwrap();

    println!("got lock");
    let tx = lock.as_ref().clone().unwrap();

    let mut metrics = prometheus::Metrics::default();
    metrics.fetch_time = Some(Instant::now());

    println!("tx");
    if let Err(e) = tx.try_send(metrics) {
        println!("Failed to send metrics - {}", e);
    }
}

fn main() {
    println!("Test");

    let (tx, rx) = mpsc::sync_channel(0);

    let mut lock = METRICS_SENDER.lock().unwrap();
    *lock = Some(tx);
    drop(lock);

    let addres = "0.0.0.0:8080".parse().unwrap();
    let fetcher = MetricsFetcher {
        requester: || export_metrics(),
        receiver: rx,
    };

    let _server = prometheus::start(addres, fetcher).unwrap();

    loop {
        std::thread::park()
    }
}
