use alloc::boxed::Box;

pub enum TaskMessage {
    Xhci,

    Dispatch(Box<dyn Fn()>),
}


impl TaskMessage {
    #[inline(always)]
    pub fn dispatch(f: impl Fn() + 'static) -> Self {
        TaskMessage::Dispatch(Box::new(f))
    }
}
