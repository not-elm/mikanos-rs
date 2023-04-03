use crate::error::{DeviceContextReason, PciError, PciResult};
use crate::xhc::device_manager::collectable::DeviceCollectable;
use crate::xhc::device_manager::device_slot::Device;
use crate::xhc::registers::traits::doorbell_registers_accessible::DoorbellRegistersAccessible;

pub struct SingleDeviceCollector<T>
where
    T: DoorbellRegistersAccessible,
{
    device_slots: u8,
    device: Option<Device<T>>,
}

impl<T> SingleDeviceCollector<T>
where
    T: DoorbellRegistersAccessible,
{
    fn check_specify_slot_id(&self, slot_id: u8) -> PciResult {
        if self.device_slots - 1 < slot_id {
            Err(PciError::FailedOperateDeviceContext(
                DeviceContextReason::ExceedMasSlots {
                    max_slots: self.device_slots,
                    specified_slot_id: slot_id,
                },
            ))
        } else {
            Ok(())
        }
    }
}

impl<T> DeviceCollectable<T> for SingleDeviceCollector<T>
where
    T: DoorbellRegistersAccessible,
{
    fn new(device_slots: u8) -> Self {
        Self {
            device_slots,
            device: None,
        }
    }
    fn mut_at(&mut self, slot_id: u8) -> Option<&mut Device<T>> {
        self.check_specify_slot_id(slot_id).ok()?;

        self.device.as_mut().and_then(|device| {
            if device.slot_id() == slot_id {
                Some(device)
            } else {
                None
            }
        })
    }

    fn set(&mut self, device_slot: Device<T>) -> PciResult {
        self.check_specify_slot_id(device_slot.slot_id())?;
        self.device = Some(device_slot);
        Ok(())
    }
}
