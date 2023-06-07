use kernel_lib::serial_println;

use crate::class_driver::ClassDriverOperate;
use crate::error::PciResult;

pub struct KeyboardDriver {
    data_buff: [i8; 8],
}


impl KeyboardDriver {
    pub fn new() -> Self {
        Self { data_buff: [0; 8] }
    }
}


impl ClassDriverOperate for KeyboardDriver {
    fn on_data_received(&mut self) -> PciResult {
        serial_println!("{:?}", self.data_buff);
        Ok(())
    }


    fn data_buff_addr(&self) -> u64 {
        self.data_buff.as_ptr() as u64
    }


    fn data_buff_len(&self) -> u32 {
        8
    }
}
