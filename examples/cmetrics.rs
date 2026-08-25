use std::{thread, time::Duration};

use concurrency::LockedMetrics;
use tracing::info;

const TASK_NUM_THREADS: usize = 2;
const REQ_NUM_THREADS: usize = 4;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let metrics = LockedMetrics::new();

    for idx in 0..TASK_NUM_THREADS {
        let metrics_clone = metrics.clone();
        thread::spawn(move || {
            loop {
                let millis = rand::random_range(100..=5000);
                thread::sleep(Duration::from_millis(millis));
                metrics_clone.inc(format!("call.thread.worker.{idx}"));
            }
        });
    }

    for page in 0..REQ_NUM_THREADS {
        let metrics_clone = metrics.clone();
        thread::spawn(move || {
            loop {
                let millis = rand::random_range(50..=800);
                thread::sleep(Duration::from_millis(millis));
                metrics_clone.inc(format!("req.page.{page}"));
            }
        });
    }

    loop {
        thread::sleep(Duration::from_secs(2));
        info!("\n{metrics}");
    }
}
