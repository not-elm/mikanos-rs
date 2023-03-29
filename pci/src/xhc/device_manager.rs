use crate::error::PciResult;
use crate::xhc::device_manager::device_collectable::DeviceCollectable;
use crate::xhc::registers::traits::port_registers_accessible::PortRegistersAccessible;

pub mod device;
pub mod device_collectable;

pub struct DeviceManager<T>
where
    T: DeviceCollectable,
{
    devices: T,
}

impl<T> DeviceManager<T>
where
    T: DeviceCollectable,
{
    pub fn new(devices: T) -> DeviceManager<T> {
        Self { devices }
    }

    pub fn address_device(
        &mut self,
        port_id: u8,
        slot_id: u8,
        registers: &mut impl PortRegistersAccessible,
    ) -> PciResult {
        self.devices
            .new_set_at(port_id, registers.read_port_speed_at(port_id)?, slot_id)?;

        Ok(())
    }
}
