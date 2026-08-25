use std::{
    sync::mpsc,
    thread::{self},
    time::Duration,
};

use anyhow::anyhow;
use tracing::info;

const NUM_PRODUCERS: usize = 4;

#[derive(Debug)]
struct Msg {
    idx: usize,
    value: usize,
}

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let (tx, rx) = mpsc::channel();

    for i in 0..NUM_PRODUCERS {
        let tx = tx.clone();
        thread::spawn(move || producer(i, tx));
    }

    drop(tx);

    let join = thread::spawn(move || {
        for msg in rx {
            info!(idx = msg.idx, value = msg.value, "received");
        }
        42
    });

    let secret = join
        .join()
        .map_err(|e| anyhow!("Thread join error: {e:?}"))?;
    info!("secret: {secret}");
    Ok(())
}

#[expect(
    clippy::needless_pass_by_value,
    reason = "producer 独占自己的 Sender，函数返回即析构，这是通道终止的前提"
)]
fn producer(idx: usize, tx: mpsc::Sender<Msg>) -> anyhow::Result<()> {
    loop {
        let value = rand::random::<u64>();
        let sleep = rand::random_range(0..5);
        let msg = Msg {
            idx,
            value: usize::try_from(value)?,
        };
        tx.send(msg)?;
        thread::sleep(Duration::from_millis(sleep));

        let num = rand::random::<u8>();
        if num.is_multiple_of(5) {
            break;
        }
    }
    Ok(())
}
