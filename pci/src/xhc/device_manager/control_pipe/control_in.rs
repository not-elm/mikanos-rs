use crate::error::PciResult;
use crate::xhc::allocator::memory_allocatable::MemoryAllocatable;
use crate::xhc::device_manager::control_pipe::request::Request;
use crate::xhc::device_manager::control_pipe::{
    make_data_stage, make_setup_stage, ControlPipeTransfer,
};
use alloc::rc::Rc;
use core::cell::RefCell;
use xhci::ring::trb::transfer::{Direction, SetupStage, StatusStage, TransferType};

use crate::xhc::device_manager::device_context_index::DeviceContextIndex;
use crate::xhc::registers::traits::doorbell_registers_accessible::DoorbellRegistersAccessible;
use crate::xhc::transfer::transfer_ring::TransferRing;

pub struct ControlIn<T>
where
    T: DoorbellRegistersAccessible,
{
    slot_id: u8,
    device_context_index: DeviceContextIndex,
    doorbell: Rc<RefCell<T>>,
    transfer_ring: TransferRing,
}

impl<T> ControlIn<T>
where
    T: DoorbellRegistersAccessible,
{
    pub fn new(
        slot_id: u8,
        device_context_index: DeviceContextIndex,
        doorbell: &Rc<RefCell<T>>,
        allocator: &mut impl MemoryAllocatable,
    ) -> PciResult<ControlIn<T>> {
        Ok(Self {
            slot_id,
            device_context_index,
            doorbell: Rc::clone(doorbell),
            transfer_ring: TransferRing::new_with_alloc(32, true, allocator)?,
        })
    }

    fn notify(&mut self) -> PciResult {
        self.doorbell.borrow_mut().notify_at(
            self.slot_id as usize,
            self.device_context_index.as_u8(),
            0,
        )
    }
}

impl<T> ControlPipeTransfer for ControlIn<T>
where
    T: DoorbellRegistersAccessible,
{
    fn no_data(&mut self, request: Request) -> PciResult {
        let setup_stage = make_setup_stage(request.into_setup_stage(), TransferType::No);
        self.transfer_ring.push(setup_stage.into_raw())?;

        let mut status = StatusStage::new();
        status.set_direction();
        status.set_interrupt_on_completion();
        self.transfer_ring.push(status.into_raw())?;
        self.notify()
    }

    fn with_data(&mut self, request: Request, buff_addr: u64, len: u32) -> PciResult {
        let setup = make_setup_stage(request.into_setup_stage(), TransferType::In);
        self.transfer_ring.push(setup.into_raw())?;

        let mut data_stage = make_data_stage(buff_addr, len, Direction::In);
        data_stage.set_interrupt_on_completion();
        self.transfer_ring.push(data_stage.into_raw())?;

        self.transfer_ring.push(StatusStage::new().into_raw())?;
        self.notify()
    }

    fn transfer_ring_base_addr(&self) -> u64 {
        self.transfer_ring.base_address()
    }
}
