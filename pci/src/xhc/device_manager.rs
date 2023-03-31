use crate::error::{DeviceContextReason, OperationReason, PciError, PciResult};
use crate::xhc::allocator::memory_allocatable::MemoryAllocatable;
use crate::xhc::device_manager::device_collectable::DeviceCollectable;
use crate::xhc::registers::traits::port_registers_accessible::PortRegistersAccessible;
use crate::xhc::transfer::device_context::DeviceContextArrayPtr;
use xhci::ring::trb::transfer::{SetupStage, StatusStage};

pub mod device;
pub mod device_collectable;
pub mod initialize_phase;

pub struct DeviceManager<T>
where
    T: DeviceCollectable,
{
    devices: T,
    device_context_array: DeviceContextArrayPtr,
    addressing_port_id: Option<u8>,
}

impl<T> DeviceManager<T>
where
    T: DeviceCollectable,
{
    pub fn new(devices: T, device_context_array: DeviceContextArrayPtr) -> DeviceManager<T> {
        Self {
            devices,
            device_context_array,
            addressing_port_id: None,
        }
    }
    pub fn set_addressing_port_id(&mut self, port_id: u8) {
        self.addressing_port_id = Some(port_id);
    }
    pub fn address_device(
        &mut self,
        slot_id: u8,
        registers: &mut impl PortRegistersAccessible,
        allocator: &mut impl MemoryAllocatable,
    ) -> PciResult<u64> {
        let port_id = self.try_addressing_port_id()?;
        self.devices.new_set_at(
            port_id,
            registers.read_port_speed_at(port_id)?,
            slot_id,
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
        device.control_in_setup_data()
    }

    pub fn initialize_at(&mut self, slot_id: u8, status_stage: StatusStage) -> PciResult {
        let device = self
            .devices
            .mut_at(slot_id)
            .ok_or(PciError::FailedOperateDeviceContext(
                DeviceContextReason::NotExistsAddressingPort,
            ))?;
        device.setup(status_stage)
    }
    fn try_addressing_port_id(&self) -> PciResult<u8> {
        self.addressing_port_id
            .ok_or(PciError::FailedOperateDeviceContext(
                DeviceContextReason::NotExistsAddressingPort,
            ))
    }
}
