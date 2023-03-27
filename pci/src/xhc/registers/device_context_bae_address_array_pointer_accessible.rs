use crate::error::PciResult;

pub trait DeviceContextBaseAddressArrayPointerAccessible {
    fn write_device_context_array_addr(&mut self, device_context_addr: u64) -> PciResult;
}
