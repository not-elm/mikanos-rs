use spin::{Mutex, MutexGuard};

use crate::interrupt;
use crate::interrupt::asm::sti_and_hlt;

#[repr(transparent)]
pub struct PreemptiveMutex<T: ?Sized>(Mutex<T>);


impl<T> PreemptiveMutex<T> {
    pub const fn new(t: T) -> PreemptiveMutex<T> {
        Self(Mutex::new(t))
    }


    pub fn lock(&self) -> MutexGuard<T> {
        loop {
            //FIXME! DEADLOCK
            if let Some(resource) = interrupt::asm::with_free(|| self.0.try_lock()) {
                return resource;
            }
            sti_and_hlt();
        }
    }
}