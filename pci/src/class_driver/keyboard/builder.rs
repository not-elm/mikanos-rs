use crate::class_driver::keyboard::driver::KeyboardDriver;
use crate::class_driver::keyboard::subscribe::KeyboardSubscribable;

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


    pub fn build<F>(self, subscribe: F) -> KeyboardDriver<F>
        where
            F: KeyboardSubscribable,
    {
        KeyboardDriver::new(self.auto_upper, subscribe)
    }
}
