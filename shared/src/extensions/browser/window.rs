use futures::channel::oneshot;
use futures::Future;
use gloo::events::EventListener;
use gloo::timers::callback::Timeout;
use std::cell::RefCell;
use std::rc::Rc;
use web_sys::Window;

pub trait WindowExtension {
    fn done_scrolling(&self) -> impl Future<Output = ()>;
}

impl WindowExtension for Window {
    async fn done_scrolling(&self) {
        let timeout_duration = 100;
        let (sender, receiver) = oneshot::channel();
        let sender = RefCell::new(Some(sender)); // Wrap sender in RefCell and Option

        let resolve = Rc::new(move || {
            let maybe_sender = sender.borrow_mut().take();
            if let Some(sender) = maybe_sender {
                sender.send(()).unwrap();
            }
        });

        let resolve_clone = resolve.clone(); // Clone resolve for the initial timeout

        let mut _timeout = Timeout::new(timeout_duration, move || {
            resolve_clone();
        });

        let event_listener = EventListener::new(self, "scroll", move |_| {
            let resolve_clone = resolve.clone(); // Clone resolve for the new timeout
            _timeout = Timeout::new(timeout_duration, move || {
                resolve_clone();
            });
        });

        receiver.await.unwrap();

        drop(event_listener);
    }
}
