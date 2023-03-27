use crate::error::PciResult;

pub trait UsbCommandRegisterAccessible {
    fn write_command_ring_addr(&mut self, command_ring_addr: u64) -> PciResult;
}
