use {
    std::{
        future::Future,
        pin::Pin,
        sync::{Arc, Mutex},
        task::{Context, Poll, Waker},
        thread,
        time::Duration,
    }
};

pub trait Service {
	type Response;
	type Error;
    type Result: Future<Output = Result<Self::Response, Self::Error>>;
    fn status(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Result>;
}