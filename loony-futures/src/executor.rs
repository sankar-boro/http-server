use {
    std::{
        future::Future,
        sync::{Arc, Mutex},
        task::{Context, Poll},
        sync::mpsc::{sync_channel, Receiver, SyncSender},
    },
};

use futures::{
    future::{BoxFuture, FutureExt},
    task::{waker_ref, ArcWake},
};

pub struct Sankar {
    current_girl: Receiver<Arc<Girl>>,
}

#[derive(Clone)]
pub struct Hooker {
    new_girl: SyncSender<Arc<Girl>>,
}
struct Girl {
    sex_in_future: Mutex<Option<BoxFuture<'static, ()>>>,
    task_sender: SyncSender<Arc<Girl>>,
}

impl Hooker {
    pub fn hook_new_girl(&self, sex: impl Future<Output = ()> + 'static + Send) {
        let sex_in_future = sex.boxed();
        let girl = Arc::new(Girl {
            sex_in_future: Mutex::new(Some(sex_in_future)),
            task_sender: self.new_girl.clone(),
        });
        self.new_girl.send(girl).expect("too many tasks queued");
    }
}
impl ArcWake for Girl {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        let cloned = arc_self.clone();
        arc_self
            .task_sender
            .send(cloned)
            .expect("too many tasks queued");
    }
}

impl Sankar {
    pub fn have_sex(&self) {
        while let Ok(girl) = self.current_girl.recv() {
            let mut future_slot = girl.sex_in_future.lock().unwrap();
            if let Some(mut future) = future_slot.take() {
                let waker = waker_ref(&girl);
                let context = &mut Context::from_waker(&*waker);
                if let Poll::Pending = future.as_mut().poll(context) {
                    *future_slot = Some(future);
                }
            }
        }
    }
}

pub fn new_channel() -> (Sankar, Hooker) {
    const MAX_QUEUED_TASKS: usize = 10_000;
    let (new_girl_receiver, current_girl) = sync_channel(MAX_QUEUED_TASKS);
    (Sankar { current_girl }, Hooker { new_girl:new_girl_receiver })
}
