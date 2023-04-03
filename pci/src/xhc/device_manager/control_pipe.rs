use alloc::rc::Rc;
use core::cell::RefCell;

use xhci::ring::trb::transfer::{DataStage, Direction, SetupStage, TransferType};

use crate::error::PciResult;
use crate::xhc::allocator::memory_allocatable::MemoryAllocatable;
use crate::xhc::device_manager::control_pipe::control_in::ControlIn;
use crate::xhc::device_manager::control_pipe::control_out::ControlOut;
use crate::xhc::device_manager::control_pipe::request::Request;
use crate::xhc::device_manager::device_context_index::DeviceContextIndex;
use crate::xhc::registers::traits::doorbell_registers_accessible::DoorbellRegistersAccessible;

mod control_in;
mod control_out;
mod request;

pub struct ControlPipe<T>
where
    T: DoorbellRegistersAccessible,
{
    control_in: ControlIn<T>,
    control_out: ControlOut<T>,
}

impl<T> ControlPipe<T>
where
    T: DoorbellRegistersAccessible,
{
    pub fn new(
        slot_id: u8,
        device_context_index: DeviceContextIndex,
        doorbell: &Rc<RefCell<T>>,
        allocator: &mut impl MemoryAllocatable,
    ) -> PciResult<Self> {
        Ok(Self {
            control_in: ControlIn::new(slot_id, device_context_index, doorbell, allocator)?,
            control_out: ControlOut::new(slot_id, device_context_index, doorbell, allocator)?,
        })
    }
}

pub trait ControlPipeTransfer {
    fn no_data(&mut self, request: Request) -> PciResult;
    fn with_data(&mut self, request: Request, data_buff_addr: u64, len: u32) -> PciResult;
    fn transfer_ring_base_addr(&self) -> u64;
}

pub(crate) fn make_setup_stage(setup_data: SetupStage, transfer_type: TransferType) -> SetupStage {
    let mut setup = SetupStage::new();
    setup.set_request_type(setup_data.request_type());
    setup.set_request(setup_data.request());
    setup.set_value(setup_data.value());
    setup.set_index(setup_data.index());
    setup.set_length(setup_data.length());
    setup.set_transfer_type(transfer_type);
    setup
}

pub(crate) fn make_data_stage(data_buff_addr: u64, length: u32, direction: Direction) -> DataStage {
    let mut data_stage = DataStage::new();

    data_stage.set_data_buffer_pointer(data_buff_addr);
    data_stage.set_trb_transfer_length(length);
    data_stage.set_td_size(0);
    data_stage.set_direction(direction);

    data_stage
}
