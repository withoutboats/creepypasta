use std::future::Future;
use std::task::{Context, Poll};

/// Attempt to resolve a future immediately. If the future is not ready, this function panics.
pub fn nosleep<F: Future>(f: F) -> F::Output {
    let waker = async_task::waker_fn(|| ());
    let mut f = Box::pin(f);
    match f.as_mut().poll(&mut Context::from_waker(&waker)) {
        Poll::Ready(output) => output,
        Poll::Pending       => panic!("future was not ready when polled"),
    }
}
