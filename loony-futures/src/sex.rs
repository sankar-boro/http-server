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