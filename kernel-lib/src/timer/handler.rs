use alloc::boxed::Box;

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


#[repr(transparent)]
pub struct TimeHandle(usize);


impl TimeHandle {
    pub fn start(
        interval: usize,
        handler: impl TimeCallback + 'static,
    ) -> Self {
        let id = TIME_HANDLE_MANAGER.entry(interval, handler);

        Self(id)
    }
}


impl Drop for TimeHandle {
    fn drop(&mut self) {
        TIME_HANDLE_MANAGER.remove(self.0);
    }
}