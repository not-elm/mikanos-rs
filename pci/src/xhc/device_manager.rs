use alloc::rc::Rc;
use core::cell::RefCell;
use core::marker::PhantomData;

use crate::class_driver::mouse::mouse_driver_factory::MouseDriverFactory;

use xhci::ring::trb::event::TransferEvent;

use crate::error::{DeviceContextReason, DeviceReason, PciError, PciResult};
use crate::xhc::allocator::memory_allocatable::MemoryAllocatable;
use crate::xhc::device_manager::collectable::DeviceCollectable;
use crate::xhc::device_manager::device::Device;
use crate::xhc::registers::traits::doorbell_registers_accessible::DoorbellRegistersAccessible;
use crate::xhc::registers::traits::port_registers_accessible::PortRegistersAccessible;
use crate::xhc::transfer::device_context::DeviceContextArrayPtr;
use crate::xhc::transfer::event::target_event::TargetEvent;

pub mod collectable;
pub mod device;

pub mod control_pipe;
pub mod descriptor;
mod device_context;
pub(crate) mod device_context_index;
pub(crate) mod endpoint_config;
pub mod endpoint_id;
pub mod initialize_phase;
mod input_context;

pub struct DeviceManager<Doorbell, Collectable, Memory>
where
    Doorbell: DoorbellRegistersAccessible + PortRegistersAccessible + 'static,
    Collectable: DeviceCollectable<Doorbell, Memory>,
    Memory: MemoryAllocatable,
{
    devices: Collectable,
    device_context_array: DeviceContextArrayPtr,
    addressing_port_id: Option<u8>,
    registers: Rc<RefCell<Doorbell>>,
    mouse_driver_factory: MouseDriverFactory,
    _maker: PhantomData<Memory>,
}

impl<Doorbell, Collectable, Memory> DeviceManager<Doorbell, Collectable, Memory>
where
    Doorbell: DoorbellRegistersAccessible + PortRegistersAccessible + 'static,
    Collectable: DeviceCollectable<Doorbell, Memory>,
    Memory: MemoryAllocatable,
{
    pub fn new(
        devices: Collectable,
        device_context_array: DeviceContextArrayPtr,
        registers: &Rc<RefCell<Doorbell>>,
        mouse_driver_factory: MouseDriverFactory,
    ) -> DeviceManager<Doorbell, Collectable, Memory> {
        Self {
            devices,
            device_context_array,
            addressing_port_id: None,
            registers: Rc::clone(registers),
            mouse_driver_factory,
            _maker: PhantomData,
        }
    }
    pub fn set_addressing_port_id(&mut self, port_id: u8) {
        self.addressing_port_id = Some(port_id);
    }
    pub fn device_slot_at(&mut self, slot_id: u8) -> Option<&mut Device<Doorbell, Memory>> {
        self.devices.mut_at(slot_id)
    }
    pub fn address_device(
        &mut self,
        slot_id: u8,
        allocator: &Rc<RefCell<Memory>>,
    ) -> PciResult<u64> {
        let port_id = self.try_addressing_port_id()?;

        let device = self.devices.new_set(
            port_id,
            self.registers.borrow().read_port_speed_at(port_id)?,
            slot_id,
            allocator,
            &self.registers,
            self.mouse_driver_factory.clone(),
        )?;

        self.device_context_array
            .set_device_context_at(slot_id as usize, device.device_context_addr());

        Ok(device.input_context_addr())
    }

    pub fn start_initialize_at(&mut self, slot_id: u8) -> PciResult {
        let device = self
            .devices
            .mut_at(slot_id)
            .ok_or(PciError::FailedOperateDeviceContext(
                DeviceContextReason::NotExistsAddressingPort,
            ))?;
        device.start_initialize()
    }
    pub fn process_transfer_event(
        &mut self,
        slot_id: u8,
        transfer_event: TransferEvent,
        target_event: TargetEvent,
    ) -> PciResult<bool> {
        let deive = self.device_mut_at(slot_id)?;
        let init_status = deive.on_transfer_event_received(transfer_event, target_event)?;
        Ok(init_status.is_initialised())
    }

    pub fn configure_endpoint(&mut self, slot_id: u8) -> PciResult {
        let device = self
            .devices
            .mut_at(slot_id)
            .ok_or(PciError::FailedOperateDeviceContext(
                DeviceContextReason::NotExistsAddressingPort,
            ))?;
        device.on_endpoints_configured()
    }

    fn try_addressing_port_id(&self) -> PciResult<u8> {
        self.addressing_port_id
            .ok_or(PciError::FailedOperateDeviceContext(
                DeviceContextReason::NotExistsAddressingPort,
            ))
    }
    fn device_mut_at(&mut self, slot_id: u8) -> PciResult<&mut Device<Doorbell, Memory>> {
        self.devices
            .mut_at(slot_id)
            .ok_or(PciError::FailedOperateDevice(DeviceReason::NotExistsSlot(
                slot_id,
            )))
    }
}
