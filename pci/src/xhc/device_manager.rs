use alloc::rc::Rc;
use core::cell::RefCell;

use xhci::ring::trb::event::TransferEvent;

use crate::error::{DeviceContextReason, PciError, PciResult};
use crate::xhc::allocator::memory_allocatable::MemoryAllocatable;
use crate::xhc::device_manager::device::Device;
use crate::xhc::device_manager::device_collectable::DeviceCollectable;
use crate::xhc::registers::traits::doorbell_registers_accessible::DoorbellRegistersAccessible;
use crate::xhc::registers::traits::port_registers_accessible::PortRegistersAccessible;
use crate::xhc::transfer::device_context::DeviceContextArrayPtr;

pub mod device;
pub mod device_collectable;

pub mod descriptor;
pub(crate) mod device_context_index;
mod endpoint_id;
pub mod initialize_phase;

pub struct DeviceManager<T, U>
where
    T: DeviceCollectable<U>,
    U: PortRegistersAccessible + DoorbellRegistersAccessible,
{
    devices: T,
    device_context_array: DeviceContextArrayPtr,
    addressing_port_id: Option<u8>,
    registers: Rc<RefCell<U>>,
}

impl<T, U> DeviceManager<T, U>
where
    T: DeviceCollectable<U>,
    U: PortRegistersAccessible + DoorbellRegistersAccessible,
{
    pub fn new(
        devices: T,
        device_context_array: DeviceContextArrayPtr,
        registers: &Rc<RefCell<U>>,
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
    pub fn device_slot_at(&mut self, slot_id: u8) -> Option<&mut Device<U>> {
        self.devices.mut_at(slot_id)
    }
    pub fn address_device(
        &mut self,
        slot_id: u8,
        allocator: &mut impl MemoryAllocatable,
    ) -> PciResult<u64> {
        let port_id = self.try_addressing_port_id()?;
        self.devices.new_set_at(
            port_id,
            self.registers.borrow().read_port_speed_at(port_id)?,
            slot_id,
            &self.registers,
            allocator,
        )?;
        let device = self.devices.mut_at(slot_id).unwrap();
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
        let device = self
            .devices
            .mut_at(slot_id)
            .ok_or(PciError::FailedOperateDeviceContext(
                DeviceContextReason::NotExistsAddressingPort,
            ))?;
        device.on_transfer_event_received(trb)?;
        Ok(device.is_init())
    }
    // pub fn initialize_at(&mut self, slot_id: u8, status_stage: StatusStage) -> PciResult {
    //     let device = self
    //         .devices
    //         .mut_at(slot_id)
    //         .ok_or(PciError::FailedOperateDeviceContext(
    //             DeviceContextReason::NotExistsAddressingPort,
    //         ))?;
    //     device.setup(status_stage)
    // }
    fn try_addressing_port_id(&self) -> PciResult<u8> {
        self.addressing_port_id
            .ok_or(PciError::FailedOperateDeviceContext(
                DeviceContextReason::NotExistsAddressingPort,
            ))
    }
}
