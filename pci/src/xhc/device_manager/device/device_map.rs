use alloc::collections::BTreeMap;
use alloc::rc::Rc;
use core::cell::RefCell;

use crate::class_driver::keyboard::driver::KeyboardDriver;
use crate::class_driver::mouse::driver::MouseDriver;
use crate::error::PciResult;
use crate::pci_error;
use crate::xhc::allocator::memory_allocatable::MemoryAllocatable;
use crate::xhc::device_manager::device::Device;
use crate::xhc::registers::traits::doorbell::DoorbellRegistersAccessible;

pub struct DeviceMap<Doorbell, Memory> {
    map: BTreeMap<u8, Device<Doorbell, Memory>>,
}

#[derive(Debug, Copy, Clone)]
pub struct DeviceConfig {
    parent_hub_slot_id: u8,
    port_speed: u8,
    slot_id: u8,
}


impl DeviceConfig {
    pub const fn new(parent_hub_slot_id: u8, port_speed: u8, slot_id: u8) -> Self {
        Self {
            parent_hub_slot_id,
            port_speed,
            slot_id,
        }
    }

    pub const fn parent_hub_slot_id(&self) -> u8 {
        self.parent_hub_slot_id
    }


    pub const fn port_speed(&self) -> u8 {
        self.port_speed
    }


    pub const fn slot_id(&self) -> u8 {
        self.slot_id
    }
}


impl<Doorbell, Memory> DeviceMap<Doorbell, Memory>
where
    Doorbell: DoorbellRegistersAccessible + 'static,
    Memory: MemoryAllocatable,
{
    pub fn new_set(
        &mut self,
        config: DeviceConfig,
        allocator: &Rc<RefCell<Memory>>,
        doorbell: &Rc<RefCell<Doorbell>>,
        mouse: MouseDriver,
        keyboard: KeyboardDriver,
    ) -> PciResult<&mut Device<Doorbell, Memory>> {
        self.set(Device::new_with_init_default_control_pipe(
            config, allocator, doorbell, mouse, keyboard,
        )?);


        self.get_mut(config.slot_id())
    }


    fn set(&mut self, device: Device<Doorbell, Memory>) {
        self.map
            .insert(device.slot_id, device);
    }


    pub fn get_mut(&mut self, slot_id: u8) -> PciResult<&mut Device<Doorbell, Memory>> {
        self.map
            .get_mut(&slot_id)
            .ok_or(pci_error!("Not found device SlotID = {slot_id}"))
    }
}


impl<Doorbell, Memory> Default for DeviceMap<Doorbell, Memory> {
    fn default() -> Self {
        Self {
            map: BTreeMap::default(),
        }
    }
}
