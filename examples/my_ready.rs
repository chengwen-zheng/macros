use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

#[tokio::main]
async fn main() {
    let fut = MyFuture::new(42);
    println!("Final value: {}", fut.await);
}

#[allow(unused)]
fn poll_future(cx: &mut Context<'_>) -> Poll<usize> {
    let mut my_future = MyFuture::new(42);
    let my_future = Pin::new(&mut my_future);
    my_ready!(my_future.poll(cx))
}

// struct myfuture

struct MyFuture {
    polled: bool,
    v: usize,
}

impl MyFuture {
    fn new(v: usize) -> Self {
        MyFuture { polled: false, v }
    }
}

impl Future for MyFuture {
    type Output = usize;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.polled {
            Poll::Ready(self.v)
        } else {
            self.polled = true;
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

#[macro_export]
macro_rules! my_ready {
    ($e:expr) => {
        match $e {
            std::task::Poll::Ready(v) => return std::task::Poll::Ready(v),
            std::task::Poll::Pending => std::task::Poll::Pending,
        }
    };
}
