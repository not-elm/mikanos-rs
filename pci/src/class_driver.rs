use crate::error::PciResult;

pub mod interrupt_in;
pub mod mouse;
pub mod boot_protocol_buffer;

pub trait ClassDriverOperate {
    fn on_data_received(&mut self) -> PciResult;
    fn data_buff_addr(&self) -> u64;
    fn data_buff_len(&self) -> u32;
}
