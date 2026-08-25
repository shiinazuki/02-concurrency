use std::{thread, time::Instant};

use concurrency::AtomicMetrics;
use tracing::info;

const NUM_THREADS: usize = 8;
const NUM_COUNT: usize = 200_000;
fn main() {
    tracing_subscriber::fmt::init();
    let keys = &["a", "b", "c"];
    let metrics = AtomicMetrics::new(keys);

    let mut joins = Vec::new();
    let now = Instant::now();
    for _ in 0..NUM_THREADS {
        let metrics = metrics.clone();
        let join = thread::spawn(move || {
            for n in 0..NUM_COUNT {
                let key = keys[n % keys.len()];
                metrics.inc(key).expect("key 已经存在 new中注册");
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
