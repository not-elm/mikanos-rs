use crate::error::PciResult;

pub mod interrupt_in;
pub mod mouse;

pub trait ClassDriverOperate {
    fn on_data_received(&mut self) -> PciResult;
    fn data_buff_addr(&self) -> u64;
    fn data_buff_len(&self) -> u32;
}

impl ClassDriverOperate for ClassDriver {
    fn on_data_received(&mut self) -> PciResult {
        match self {
            Self::Mouse(m) => m.on_data_received(),
        }
    }

    fn data_buff_addr(&self) -> u64 {
        match self {
            Self::Mouse(m) => m.data_buff_addr(),
        }
    }

    fn data_buff_len(&self) -> u32 {
        match self {
            Self::Mouse(m) => m.data_buff_len(),
        }
    }
}
