use std::{
    pin::{Pin, pin},
    sync::Arc,
    task::{Context, Poll, Wake, Waker},
    thread::{self, Thread},
};

use tracing::info;

/// 一个「被 poll 若干次之后才完成」的 Future。
struct CountDown {
    remaining: u32,
    wake: bool,
}

impl Future for CountDown {
    type Output = &'static str;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        info!("     [poll] remaining = {}", self.remaining);
        if self.remaining == 0 {
            return Poll::Ready("Done");
        }
        self.remaining -= 1;
        if self.wake {
            cx.waker().wake_by_ref();
        }
        Poll::Pending
    }
}

/// 执行器 A：忙轮询。无视 Waker，Pending 了就立刻重试
fn busy_block_on<F: Future>(fut: F) -> F::Output {
    let mut cx = Context::from_waker(Waker::noop());
    let mut fut = pin!(fut);
    let mut round = 0;
    loop {
        round += 1;
        info!("     执行器 A 第 {round} 轮");
        if let Poll::Ready(v) = fut.as_mut().poll(&mut cx) {
            return v;
        }
    }
}

/// 执行器 B 用的 Waker：把某个线程叫醒
struct ThreadWaker(Thread);

impl Wake for ThreadWaker {
    fn wake(self: Arc<Self>) {
        self.0.unpark();
    }

    fn wake_by_ref(self: &Arc<Self>) {
        self.0.unpark();
    }
}

/// 执行器 B：会挂起。Pending 就睡过去，等 Waker 来叫
fn block_on<F: Future>(fut: F) -> F::Output {
    let waker = Waker::from(Arc::new(ThreadWaker(thread::current())));
    let mut cx = Context::from_waker(&waker);
    let mut fut = pin!(fut);
    let mut round = 0;
    loop {
        round += 1;
        info!("     执行器 B 第 {round} 轮");
        match fut.as_mut().poll(&mut cx) {
            Poll::Ready(v) => return v,
            Poll::Pending => thread::park(),
        }
    }
}

fn main() {
    tracing_subscriber::fmt::init();
    info!("=== 执行器 A（忙轮询）===");
    info!(
        "结果: {}\n",
        busy_block_on(CountDown {
            remaining: 3,
            wake: true
        })
    );

    info!("=== 执行器 B（会挂起）===");
    info!(
        "结果: {}",
        block_on(CountDown {
            remaining: 3,
            wake: true
        })
    );
}
