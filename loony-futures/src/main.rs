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
use std::borrow::{Borrow, BorrowMut};

use futures::{future::{BoxFuture, FutureExt}, task::{waker_ref, ArcWake}};

struct Sender {
    shared: StdSender<Arc<Task>>
}
struct Receiver {
    shared: StdReceiver<Arc<Task>>
}

pub struct Task {
    task: Mutex<Option<BoxFuture<'static, ()>>>,
    sender: StdSender<Arc<Task>>,
}

impl Sender {
    /// Sends the provided message along this channel.
    pub fn send(&self, task: impl Future<Output = ()> + 'static + Send) {
       let _task = task.boxed();
        let girl = Arc::new(Task {
            task: Mutex::new(Some(_task)),
            sender: self.shared.clone(),
        });
        self.shared.send(girl).expect("too many tasks queued");
    }
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        let cloned = arc_self.clone();
        println!("wake_by_ref");
        arc_self
            .sender
            .send(cloned)
            .expect("too many tasks queued");
    }

    fn wake(self: Arc<Self>) {
        Self::wake_by_ref(&self)
    }
}

impl Receiver {
    pub fn work(&self) {
        while let Ok(task) = self.shared.recv() {
            let mut future_slot = task.task.lock().unwrap();
            if let Some(mut future) = future_slot.take() {
                let waker = waker_ref(&task);
                let context = &mut Context::from_waker(&*waker);
                if let Poll::Pending = future.as_mut().poll(context) {
                    *future_slot = Some(future);
                }
             }
        }
    }
}

fn channel() -> (Sender, Receiver) {
    const MAX_QUEUED_TASKS: usize = 10_000;
    let (sender, receiver) = sync_channel::<Arc<Task>>(MAX_QUEUED_TASKS);
    (Sender { shared: sender }, Receiver { shared:receiver })
}

pub struct Sex {
    girlfriend: Arc<Mutex<GirlFriend>>,
}

struct GirlFriend {
    is_hot: bool,
    on_search: Option<Waker>,
}

impl Future for Sex {
    type Output = ();

    fn poll(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        println!("Poll called.");
        let mut gf = self.girlfriend.lock().unwrap();
        if gf.is_hot {
            Poll::Ready(())
        } else {
            gf.on_search = Some(context.waker().clone());
            Poll::Pending
        }
    }
}

impl Sex {
    pub fn duration(duration: Duration) -> Self {
        let new_gf = Arc::new(Mutex::new(GirlFriend {
            is_hot: false,
            on_search: None,
        }));
        let current_gf = new_gf.clone();
        thread::spawn(move || {
            thread::sleep(duration);
            let mut c_gf = current_gf.lock().unwrap();
            c_gf.is_hot = true;
            if let Some(sher_mode) = c_gf.on_search.take() {
                sher_mode.wake()
            }
        });
        Sex { girlfriend:new_gf }
    }
}
fn main() {
    let (sender, receiver) = channel();
    sender.send(async {
        println!("Hello World.");
        Sex::duration(Duration::new(2, 0)).await;
        println!("I have printed after two seconds.");
    });
    sender.send(async {
        println!("Hello World!");
        Sex::duration(Duration::new(5, 0)).await;
        println!("I have printed after 5 seconds.");
    });
    receiver.work();
}