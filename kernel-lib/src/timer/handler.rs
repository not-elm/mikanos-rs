use alloc::boxed::Box;
use alloc::rc::Rc;

use crate::task::message::TaskMessage;
use crate::task::TASK_MANAGER;
use crate::timer::TIME_HANDLE_MANAGER;

pub mod timer;
pub mod manager;

pub type BoxedTimeHandler = Box<dyn TimeCallback>;


pub trait TimeCallback {
    fn call(&self);
}


impl<F> TimeCallback for F where F: Fn() {
    fn call(&self) {
        self();
    }
}

pub struct SharedTimeCallback(Rc<dyn TimeCallback>);


impl SharedTimeCallback {
    pub fn new(handler: impl TimeCallback + 'static) -> Self {
        Self(Rc::new(handler))
    }
}


impl TimeCallback for SharedTimeCallback {
    fn call(&self) {
        self.0.call();
    }
}


impl Clone for SharedTimeCallback {
    fn clone(&self) -> Self {
        Self(Rc::clone(&self.0))
    }
}


pub struct TimeHandle {
    id: usize,

}


impl TimeHandle {
    pub fn start(
        interval: usize,
        handler: impl TimeCallback + 'static,
    ) -> Self {
        let id = TIME_HANDLE_MANAGER.entry(interval, handler);

        Self {
            id,

        }
    }


    pub fn start_dispatch_on_main(
        interval: usize,
        handler: impl TimeCallback + 'static,
    ) -> Self {
        let handler = SharedTimeCallback::new(handler);
        let id = TIME_HANDLE_MANAGER.entry(interval, move || {
            unsafe {
                let h = handler.clone();

                TASK_MANAGER.send_message_at(0, TaskMessage::dispatch(move || {
                    h.call();
                })).unwrap();
            }
        });

        Self {
            id,

        }
    }
}


impl Drop for TimeHandle {
    fn drop(&mut self) {
        TIME_HANDLE_MANAGER.remove(self.id);
    }
}