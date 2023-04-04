use crate::class_driver::mouse::mouse_default_driver::MouseDefaultDriver;
use crate::class_driver::mouse::mouse_subscribe_driver::{MouseSubscribeDriver, MouseSubscriber};
use crate::class_driver::ClassDriverOperate;
use alloc::boxed::Box;

pub enum MouseDriverFactory<T: MouseSubscriber> {
    Default,
    Subscribe(T),
}

impl<T: MouseSubscriber> MouseDriverFactory<T> {
    pub fn subscriber(subscriber: T) -> Self {
        Self::Subscribe(subscriber)
    }
}
impl<T: MouseSubscriber + Clone> MouseDriverFactory<T> {
    pub fn fact(&self) -> Box<dyn ClassDriverOperate> {
        match self {
            Self::Default => Box::new(MouseDefaultDriver::new()),
            Self::Subscribe(subscriber) => Box::new(MouseSubscribeDriver::new(subscriber.clone())),
        }
    }
}

impl<T: MouseSubscriber> Default for MouseDriverFactory<T> {
    fn default() -> Self {
        Self::Default
    }
}
