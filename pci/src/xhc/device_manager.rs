use alloc::rc::Rc;
use core::cell::RefCell;

use xhci::ring::trb::event::TransferEvent;

use device::device_map::DeviceMap;

use crate::class_driver::keyboard::driver::KeyboardDriver;
use crate::class_driver::mouse::mouse_driver_factory::MouseDriverFactory;
use crate::error::PciResult;
use crate::pci_error;
use crate::xhc::allocator::memory_allocatable::MemoryAllocatable;
use crate::xhc::device_manager::device::Device;
use crate::xhc::registers::traits::doorbell::DoorbellRegistersAccessible;
use crate::xhc::registers::traits::port::PortRegistersAccessible;
use crate::xhc::transfer::device_context::DeviceContextArrayPtr;
use crate::xhc::transfer::event::target_event::TargetEvent;

pub mod control_pipe;
pub mod descriptor;
pub mod device;
mod device_context;
pub(crate) mod device_context_index;
pub(crate) mod endpoint_config;
pub mod endpoint_id;
pub mod initialize_phase;
mod input_context;

pub struct DeviceManager<Doorbell, Memory> {
    devices: DeviceMap<Doorbell, Memory>,
    device_context_array: DeviceContextArrayPtr,
    addressing_port_id: Option<u8>,
    registers: Rc<RefCell<Doorbell>>,
    mouse_driver_factory: MouseDriverFactory,
}


impl<Doorbell, Memory> DeviceManager<Doorbell, Memory>
where
    Doorbell: DoorbellRegistersAccessible + PortRegistersAccessible + 'static,
    Memory: MemoryAllocatable,
{
    pub fn new(
        devices: DeviceMap<Doorbell, Memory>,
        device_context_array: DeviceContextArrayPtr,
        registers: &Rc<RefCell<Doorbell>>,
        mouse_driver_factory: MouseDriverFactory,
    ) -> DeviceManager<Doorbell, Memory> {
        Self {
            devices,
            device_context_array,
            addressing_port_id: None,
            registers: Rc::clone(registers),
            mouse_driver_factory,
        }
    }


    pub fn address_port(&self, port: u8) -> bool {
        if let Some(addressing_port) = self.addressing_port_id {
            port == addressing_port
        } else {
            true
        }
    }


    pub fn set_addressing_port_id(&mut self, port_id: u8) {
        self.addressing_port_id = Some(port_id);
    }


    pub fn device_slot_at(&mut self, slot_id: u8) -> PciResult<&mut Device<Doorbell, Memory>> {
        self.devices.get_mut(slot_id)
    }


    pub fn address_device(
        &mut self,
        slot_id: u8,
        allocator: &Rc<RefCell<Memory>>,
    ) -> PciResult<u64> {
        let parent_hub_slot_id = self.try_addressing_port_id()?;

        let device = self.new_device(parent_hub_slot_id, slot_id, allocator)?;
        let device_context_addr = device.device_context_addr();
        let input_context_addr = device.input_context_addr();

        self.device_context_array
            .set_device_context_at(slot_id as usize, device_context_addr);

        self.addressing_port_id = None;

        Ok(input_context_addr)
    }


    pub fn start_initialize_at(&mut self, slot_id: u8) -> PciResult {
        let device = self
            .devices
            .get_mut(slot_id)?;

        device.start_initialize()
    }


    pub fn process_transfer_event(
        &mut self,
        slot_id: u8,
        transfer_event: TransferEvent,
        target_event: TargetEvent,
        keyboard: KeyboardDriver,
    ) -> PciResult<bool> {
        let device = self.device_mut_at(slot_id)?;
        let init_status =
            device.on_transfer_event_received(transfer_event, target_event, keyboard)?;

        Ok(init_status.is_initialised())
    }


    pub fn configure_endpoint(&mut self, slot_id: u8) -> PciResult {
        let device = self
            .devices
            .get_mut(slot_id)?;

        device.on_endpoints_configured()
    }


    fn new_device(
        &mut self,
        parent_hub_slot_id: u8,
        slot_id: u8,
        allocator: &Rc<RefCell<Memory>>,
    ) -> PciResult<&mut Device<Doorbell, Memory>> {
        let port_speed = self
            .registers
            .borrow()
            .read_port_speed_at(parent_hub_slot_id)?;

        self.devices.new_set(
            parent_hub_slot_id,
            port_speed,
            slot_id,
            allocator,
            &self.registers,
            self.mouse_driver_factory
                .clone(),
        )
    }


    fn try_addressing_port_id(&self) -> PciResult<u8> {
        self.addressing_port_id
            .ok_or(pci_error!("Not exists addressing port"))
    }


    fn device_mut_at(&mut self, slot_id: u8) -> PciResult<&mut Device<Doorbell, Memory>> {
        self.devices.get_mut(slot_id)
    }
}
