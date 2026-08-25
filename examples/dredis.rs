use std::{net::SocketAddr, time::Duration};

use anyhow::Result;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    task::JoinSet,
};
use tracing::{info, warn};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let listener = TcpListener::bind("0.0.0.0:6379").await?;
    let mut conns = JoinSet::new();
    loop {
        tokio::select! {
            r = listener.accept() => {
                let (stream, raddr ) = r?;
                conns.spawn(async move { handle(stream, raddr).await});
            }
            _ = tokio::signal::ctrl_c() => {
                info!("收到停机信号，停止接受新连接");
                          break;
            }
        }
    }

    info!("等待 {} 个在途连接收尾", conns.len());
    while let Some(res) = conns.join_next().await {
        match res {
            Ok(Ok(())) => {}
            Ok(Err(e)) => warn!("连接处理出错: {e}"),
            Err(e) => warn!("任务 panic: {e}"),
        }
    }
    info!("干净退出");

    Ok(())
}

async fn handle(mut stream: TcpStream, raddr: SocketAddr) -> Result<()> {
    info!("accept {raddr}");
    let mut buf = [0u8; 4096];
    loop {
        let n = stream.read(&mut buf).await?;
        if n == 0 {
            break;
        }
        info!("{}", String::from_utf8_lossy(&buf[0..n]));
        tokio::time::sleep(Duration::from_secs(5)).await; // 假装这个请求很慢
        stream.write_all(b"+OK\r\n").await?;
    }
    Ok(())
}
