use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use anyhow::Result;
use tokio::time::{Instant, sleep};
use tracing::info;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let mut handles = Vec::new();
    let now = Instant::now();
    let m = Arc::new(Mutex::new(40));

    for _ in 0..4 {
        let m = m.clone();
        let join = tokio::spawn(async move {
            tokio::spawn(hold_guard(m)).await.expect("error");
        });
        handles.push(join);
    }

    for join in handles {
        join.await?;
    }
    let elapsed = now.elapsed();
    info!("{elapsed:?}");
    Ok(())
}

async fn hold_guard(m: Arc<Mutex<i32>>) {
    let value = *m.lock().expect("aa");
    sleep(Duration::from_millis(10)).await;
    info!("{value}");
}
