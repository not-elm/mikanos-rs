use spin::{Mutex, MutexGuard};

use common_lib::assembly::hlt;

use crate::interrupt;

#[repr(transparent)]
pub struct PreemptiveMutex<T: ?Sized>(Mutex<T>);


impl<T> PreemptiveMutex<T> {
    pub const fn new(t: T) -> PreemptiveMutex<T> {
        Self(Mutex::new(t))
    }


    pub fn lock(&self) -> MutexGuard<T> {
        loop {
            if let Some(resource) = interrupt::asm::with_free(|| self.0.try_lock()) {
                return resource;
            }
            hlt();
        }
    }
}
