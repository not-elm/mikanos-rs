use alloc::boxed::Box;
use alloc::string::String;

pub enum TaskMessage {
    Xhci,

    Keyboard,

    Count {
        layer_key: String,
        count: usize,
    },

    Print {
        layer_key: String,
        text: String,
    },

    Dispatch(Box<dyn Fn()>),
}


impl TaskMessage {
    #[inline(always)]
    pub const fn count(layer_key: String, count: usize) -> Self {
        Self::Count {
            layer_key,
            count,
        }
    }


    #[inline(always)]
    pub const fn print(layer_key: String, text: String) -> Self {
        Self::Print {
            layer_key,
            text,
        }
    }


    #[inline(always)]
    pub fn dispatch(f: impl Fn() + 'static) -> Self {
        TaskMessage::Dispatch(Box::new(f))
    }
}
