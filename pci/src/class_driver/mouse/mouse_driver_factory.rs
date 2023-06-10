use alloc::boxed::Box;
use alloc::rc::Rc;

use crate::class_driver::mouse::mouse_default_driver::MouseDefaultDriver;
use crate::class_driver::mouse::mouse_subscribe_driver::MouseSubscribeDriver;
use crate::class_driver::ClassDriverOperate;

use super::mouse_subscribable::MouseSubscribable;
#[derive(Clone)]
pub enum MouseDriverFactory {
    Default,
    Subscribe(Rc<dyn MouseSubscribable>),
}

impl MouseDriverFactory {
    pub fn subscriber(subscriber: impl MouseSubscribable + 'static) -> Self {
        Self::Subscribe(Rc::new(subscriber))
    }
}

impl MouseDriverFactory {
    pub fn fact(&self) -> Box<dyn ClassDriverOperate> {
        match self {
            Self::Default => Box::new(MouseDefaultDriver::new()),
            Self::Subscribe(subscriber) => Box::new(MouseSubscribeDriver::new(subscriber)),
        }
    }
}

impl Default for MouseDriverFactory {
    fn default() -> Self {
        Self::Default
    }
}
