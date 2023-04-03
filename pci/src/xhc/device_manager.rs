use alloc::rc::Rc;
use core::cell::RefCell;

use xhci::ring::trb::event::TransferEvent;

use crate::error::{DeviceContextReason, DeviceReason, PciError, PciResult};
use crate::xhc::allocator::memory_allocatable::MemoryAllocatable;
use crate::xhc::device_manager::collectable::DeviceCollectable;
use crate::xhc::device_manager::device_slot::Device;
use crate::xhc::registers::traits::doorbell_registers_accessible::DoorbellRegistersAccessible;
use crate::xhc::registers::traits::port_registers_accessible::PortRegistersAccessible;
use crate::xhc::transfer::device_context::DeviceContextArrayPtr;

pub mod collectable;
pub mod device_slot;

pub mod control_pipe;
pub mod descriptor;
pub(crate) mod device_context_index;
mod endpoint_config;
pub mod endpoint_id;
pub mod initialize_phase;

pub struct DeviceManager<T, U>
where
    T: DoorbellRegistersAccessible + PortRegistersAccessible,
    U: DeviceCollectable<T>,
{
    devices: U,
    device_context_array: DeviceContextArrayPtr,
    addressing_port_id: Option<u8>,
    registers: Rc<RefCell<T>>,
}

impl<T, U> DeviceManager<T, U>
where
    T: DoorbellRegistersAccessible + PortRegistersAccessible,
    U: DeviceCollectable<T>,
{
    pub fn new(
        devices: U,
        device_context_array: DeviceContextArrayPtr,
        registers: &Rc<RefCell<T>>,
    ) -> DeviceManager<T, U> {
        Self {
            devices,
            device_context_array,
            addressing_port_id: None,
            registers: Rc::clone(registers),
        }
    }
    pub fn set_addressing_port_id(&mut self, port_id: u8) {
        self.addressing_port_id = Some(port_id);
    }
    pub fn device_slot_at(&mut self, slot_id: u8) -> Option<&mut Device<T>> {
        self.devices.mut_at(slot_id)
    }
    pub fn address_device(
        &mut self,
        slot_id: u8,
        allocator: &mut impl MemoryAllocatable,
    ) -> PciResult<u64> {
        let port_id = self.try_addressing_port_id()?;

        let device = self.devices.new_set(
            port_id,
            self.registers.borrow().read_port_speed_at(port_id)?,
            slot_id,
            allocator,
            &self.registers,
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
    pub fn initialize_phase_at(&mut self, slot_id: u8, trb: TransferEvent) -> PciResult<bool> {
        let device = self.device_mut_at(slot_id)?;
        device.on_transfer_event_received(trb)?;
        Ok(device.is_init())
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
    fn device_mut_at(&mut self, slot_id: u8) -> PciResult<&mut Device<T>> {
        self.devices
            .mut_at(slot_id)
            .ok_or(PciError::FailedOperateDevice(DeviceReason::NotExistsSlot(
                slot_id,
            )))
    }
}
