use std::{
    sync::mpsc::{self, Sender},
    thread,
    time::Duration,
};

use anyhow::Result;

const PROVIDER_NUM: usize = 4;

#[allow(dead_code)]
#[derive(Debug)]
struct Msg {
    idx: usize,
    value: usize,
}

fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel();

    for idx in 0..PROVIDER_NUM {
        let tx = tx.clone();
        thread::spawn(move || provide(idx, tx));
    }

    drop(tx);
    let consumer = thread::spawn(|| {
        for item in rx {
            println!("consumer: {:?}", item);
        }
        44
    });

    let consumer = consumer.join().map_err(|e| anyhow::anyhow!("{:?}", e))?;

    println!("consumer end {}", consumer);

    Ok(())
}

fn provide(idx: usize, tx: Sender<Msg>) -> Result<()> {
    loop {
        let value = rand::random::<u32>();
        let msg = Msg::new(idx, value as _);
        tx.send(msg)?;
        let sleep_time = rand::random::<u8>() as u64 * 10;
        thread::sleep(Duration::from_millis(sleep_time));
        if value.is_multiple_of(5) {
            println!("idx {} end", idx);
            break;
        }
    }
    Ok(())
}

impl Msg {
    fn new(idx: usize, value: usize) -> Self {
        Self { idx, value }
    }
}
