use alloc::boxed::Box;
use core::sync::atomic::AtomicUsize;
use core::sync::atomic::Ordering::Relaxed;

use crate::timer::handler::{BoxedTimeHandler, TimeCallback};

pub struct HandleTimer {
    interval: usize,
    tick: AtomicUsize,
    handler: BoxedTimeHandler,
}


impl HandleTimer {
    #[inline(always)]
    pub fn new(interval: usize, handler: impl TimeCallback + 'static) -> Self {
        Self {
            interval,
            tick: AtomicUsize::new(0),
            handler: Box::new(handler),
        }
    }


    pub fn tick(&self) {
        let next_tick = self
            .tick
            .fetch_add(1, Relaxed);

        if self.interval <= next_tick {
            self.reset();
            self.handler.call();
        }
    }


    #[inline(always)]
    pub fn reset(&self) {
        self.tick.store(0, Relaxed);
    }
}


