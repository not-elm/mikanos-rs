use alloc::collections::BTreeMap;
use core::cell::RefCell;
use core::sync::atomic::{AtomicUsize, Ordering};

use crate::timer::handler::TimeCallback;
use crate::timer::handler::timer::HandleTimer;

pub struct TimeHandleManager {
    handlers: RefCell<BTreeMap<usize, HandleTimer>>,
}


unsafe impl Sync for TimeHandleManager {}


impl TimeHandleManager {
    #[inline]
    pub const fn new() -> Self {
        Self {
            handlers: RefCell::new(BTreeMap::new())
        }
    }


    #[inline]
    pub fn tick(&self) {
        self.handlers
            .borrow()
            .values()
            .for_each(|timer| {
                timer.tick();
            })
    }


    pub fn entry(&self, interval: usize, handler: impl TimeCallback + 'static) -> usize {
        static ID: AtomicUsize = AtomicUsize::new(0);
        let id = ID.fetch_add(1, Ordering::Relaxed);

        self.handlers
            .borrow_mut()
            .insert(id, HandleTimer::new(interval, handler));

        id
    }


    pub fn remove(&self, id: usize) {
        self.handlers
            .borrow_mut()
            .remove(&id);
    }
}
