use alloc::rc::Rc;

use crate::class_driver::keyboard::driver::KeyboardDriver;
use crate::class_driver::keyboard::subscribe::KeyboardSubscribable;
use crate::class_driver::keyboard::Keycode;

#[derive(Debug)]
pub struct Builder {
    auto_upper: bool,
}


impl Builder {
    pub const fn new() -> Self {
        Self { auto_upper: false }
    }


    pub fn auto_upper_if_shift(mut self) -> Self {
        self.auto_upper = true;
        self
    }


    pub fn boxed_build<F>(self, subscribe: F) -> KeyboardDriver
    where
        F: KeyboardSubscribable + 'static,
    {
        KeyboardDriver::new(self.auto_upper, Rc::new(subscribe))
    }


    #[cfg(test)]
    pub(crate) fn mock(self) -> KeyboardDriver {
        self.boxed_build(MockSubscriber)
    }
}

#[cfg(test)]
pub struct MockSubscriber;

#[cfg(test)]
impl KeyboardSubscribable for MockSubscriber {
    fn subscribe(&self, _: u8, _: Keycode) {}
}
