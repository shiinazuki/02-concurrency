use std::{thread, time::Instant};

use concurrency::ShardedMetrics;
use tracing::info;

const NUM_THREADS: usize = 8;
const NUM_COUNT: usize = 200_000_000;
fn main() {
    tracing_subscriber::fmt::init();
    let metrics = ShardedMetrics::default();

    let mut joins = Vec::new();
    let now = Instant::now();
    for _ in 0..NUM_THREADS {
        let metrics = metrics.clone();
        let join = thread::spawn(move || {
            for n in 0..NUM_COUNT {
                metrics.inc(format!("key.{}", n % 64));
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
