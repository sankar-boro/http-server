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

/// Shared state between the future and the waiting thread
struct GirlFriend {
    /// Whether or not the sleep time has elapsed
    is_hot: bool,

    /// The waker for the task that `TimerFuture` is running on.
    /// The thread can use this after setting `completed = true` to tell
    /// `TimerFuture`'s task to wake up, see that `completed = true`, and
    /// move forward.
    on_search: Option<Waker>,
}

impl Future for Sex {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Look at the shared state to see if the timer has already completed.
        let mut gf = self.girlfriend.lock().unwrap();
        if gf.is_hot {
            Poll::Ready(())
        } else {
            // Set waker so that the thread can wake up the current task
            // when the timer has completed, ensuring that the future is polled
            // again and sees that `completed = true`.
            //
            // It's tempting to do this once rather than repeatedly cloning
            // the waker each time. However, the `TimerFuture` can move between
            // tasks on the executor, which could cause a stale waker pointing
            // to the wrong task, preventing `TimerFuture` from waking up
            // correctly.
            //
            // N.B. it's possible to check for this using the `Waker::will_wake`
            // function, but we omit that here to keep things simple.
            gf.on_search = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

impl Sex {
    /// Create a new `TimerFuture` which will complete after the provided
    /// timeout.
    pub fn duration(duration: Duration) -> Self {
        let new_gf = Arc::new(Mutex::new(GirlFriend {
            is_hot: false,
            on_search: None,
        }));

        // Spawn the new thread
        let current_gf = new_gf.clone();
        thread::spawn(move || {
            thread::sleep(duration);
            let mut c_gf = current_gf.lock().unwrap();
            // Signal that the timer has completed and wake up the last
            // task on which the future was polled, if one exists.
            c_gf.is_hot = true;
            if let Some(sher_mode) = c_gf.on_search.take() {
                sher_mode.wake()
            }
        });

        Sex { girlfriend:new_gf }
    }
}