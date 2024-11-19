use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // let mut ctx = Context::from_waker(futures::task::noop_waker_ref());
    // let v = pool_future(&mut ctx);
    // println!("{:?}", v);

    let fut = MyFuture::new(42);

    let a = fut.await;
    println!("{:?}", a);
    Ok(())
}

#[allow(dead_code)]
fn pool_future(ctx: &mut Context<'_>) -> Poll<usize> {
    let mut fut = MyFuture::new(42);
    let fut = Pin::new(&mut fut);

    std::task::Poll::Ready(my_ready!(fut.poll(ctx)))
}

struct MyFuture {
    polled: bool,
    value: usize,
}

impl MyFuture {
    fn new(value: usize) -> Self {
        Self {
            value,
            polled: false,
        }
    }
}

impl Future for MyFuture {
    type Output = usize;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.polled {
            Poll::Ready(self.value)
        } else {
            self.polled = true;
            // 唤醒当前任务
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

#[macro_export]
macro_rules! my_ready {
    ($expr:expr) => {
        match $expr {
            std::task::Poll::Ready(v) => v,
            std::task::Poll::Pending => return std::task::Poll::Pending,
        }
    };
}
