use spin::{Mutex, MutexGuard};


use crate::interrupt;
use crate::task::TASK_MANAGER;

#[repr(transparent)]
pub struct PreemptiveMutex<T: ?Sized>(Mutex<T>);


impl<T> PreemptiveMutex<T> {
    pub const fn new(t: T) -> PreemptiveMutex<T> {
        Self(Mutex::new(t))
    }


    pub fn lock(&self) -> MutexGuard<T> {
        loop {
            if let Some(resource) = interrupt::asm::without_interrupt(|| self.0.try_lock()) {
                return resource;
            }
            unsafe {
                TASK_MANAGER
                    .switch_ignore_priority()
                    .unwrap();
            }
        }
    }
}
