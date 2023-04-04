use crate::error::{DeviceContextReason, PciError, PciResult};
use crate::xhc::allocator::memory_allocatable::MemoryAllocatable;
use crate::xhc::device_manager::collectable::DeviceCollectable;
use crate::xhc::device_manager::device::Device;
use crate::xhc::registers::traits::doorbell_registers_accessible::DoorbellRegistersAccessible;

pub struct SingleDeviceCollector<Doorbell, Memory>
where
    Doorbell: DoorbellRegistersAccessible,
    Memory: MemoryAllocatable,
{
    device_slots: u8,
    device: Option<Device<Doorbell, Memory>>,
}

impl<Doorbell, Memory> SingleDeviceCollector<Doorbell, Memory>
where
    Doorbell: DoorbellRegistersAccessible,
    Memory: MemoryAllocatable,
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

impl<T, Memory> DeviceCollectable<T, Memory> for SingleDeviceCollector<T, Memory>
where
    T: DoorbellRegistersAccessible + 'static,
    Memory: MemoryAllocatable,
{
    fn new(device_slots: u8) -> Self {
        Self {
            device_slots,
            device: None,
        }
    }
    fn mut_at(&mut self, slot_id: u8) -> Option<&mut Device<T, Memory>> {
        self.check_specify_slot_id(slot_id).ok()?;

        self.device.as_mut().and_then(|device| {
            if device.slot_id() == slot_id {
                Some(device)
            } else {
                None
            }
        })
    }

    fn set(&mut self, device_slot: Device<T, Memory>) -> PciResult {
        self.check_specify_slot_id(device_slot.slot_id())?;
        self.device = Some(device_slot);
        Ok(())
    }
}
