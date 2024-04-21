use async_std::prelude::*;
mod service;

use {
    std::{
        pin::Pin,
        future::Future,
        sync::{Arc, Mutex},
        task::{Context, Poll, Waker},
        sync::mpsc::{sync_channel, SyncSender as StdSender, Receiver as StdReceiver},
        thread,
        time::Duration,
    },
};
use futures::{future::{BoxFuture, FutureExt}, task::{waker_ref, ArcWake}};
use async_std::task;

trait Factory<R> {
    fn call(&self) -> R;
}

impl<T, R> Factory<R> for T where T: Fn() -> R, R: Future<Output=String> {
    fn call(&self) -> R {
        self()
    }
}

async fn run<T, R>(factory: T) -> String where T: Factory<R>, R: Future<Output=String> {
    task::block_on(factory.call())
}

async fn index() -> String {
    "Sankar".to_string()
}

#[async_std::main]
async fn main() {
    println!("Starting app.");
    let data = run(index).await;
    println!("{}", data);
}
