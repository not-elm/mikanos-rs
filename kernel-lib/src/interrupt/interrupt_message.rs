use alloc::string::String;

#[derive(Debug, Clone, Eq, PartialEq)]
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
}
