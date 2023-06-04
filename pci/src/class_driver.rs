use crate::error::OldPciResult;

pub mod boot_protocol_buffer;
pub mod interrupt_in;
pub mod mouse;

pub trait ClassDriverOperate {
    fn on_data_received(&mut self) -> OldPciResult;
    fn data_buff_addr(&self) -> u64;
    fn data_buff_len(&self) -> u32;
}
