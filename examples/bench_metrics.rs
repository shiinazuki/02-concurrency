use std::{thread, time::Instant};

use concurrency::LockedMetrics;
use tracing::info;

const NUM_THREADS: usize = 8;
const NUM_COUNT: usize = 200_000;
fn main() {
    tracing_subscriber::fmt::init();
    let metrics = LockedMetrics::default();

    let mut joins = Vec::new();
    let now = Instant::now();
    for _ in 0..NUM_THREADS {
        let metrics = metrics.clone();
        let join = thread::spawn(move || {
            for n in 0..NUM_COUNT {
                // metrics.inc("hot{}");
                metrics.inc(format!("key.{}", n % 16));
            }
        });
        joins.push(join);
    }
    for join in joins {
        join.join().expect("thread panicked");
    }
    let last = now.elapsed();

    info!("data: {:?}, metrics: {}", last, metrics);
}
