use crate::error::PciResult;

pub trait ConfigRegisterAccessible {
    fn write_max_device_slots_enabled(&mut self, max_device_slots: u8) -> PciResult;
}
