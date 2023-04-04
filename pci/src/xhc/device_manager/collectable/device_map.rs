use crate::error::PciResult;
use crate::xhc::allocator::memory_allocatable::MemoryAllocatable;
use crate::xhc::device_manager::collectable::DeviceCollectable;
use crate::xhc::device_manager::control_pipe::unstable_hash_map::UnstableHashMap;
use crate::xhc::device_manager::device::Device;
use crate::xhc::registers::traits::doorbell_registers_accessible::DoorbellRegistersAccessible;

pub struct DeviceMap<Doorbell, Memory>
where
    Doorbell: DoorbellRegistersAccessible + 'static,
    Memory: MemoryAllocatable,
{
    map: UnstableHashMap<u8, Device<Doorbell, Memory>>,
}

impl<Doorbell, Memory> DeviceCollectable<Doorbell, Memory> for DeviceMap<Doorbell, Memory>
where
    Doorbell: DoorbellRegistersAccessible + 'static,
    Memory: MemoryAllocatable,
{
    fn new(_: u8) -> Self {
        Self {
            map: UnstableHashMap::new(),
        }
    }

    fn mut_at(&mut self, slot_id: u8) -> Option<&mut Device<Doorbell, Memory>> {
        self.map.get_mut(slot_id)
    }

    fn set(&mut self, device: Device<Doorbell, Memory>) -> PciResult {
        self.map.set(device.slot_id(), device);
        Ok(())
    }
}
