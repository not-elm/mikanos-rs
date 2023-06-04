use alloc::rc::Rc;
use core::cell::RefCell;

use xhci::ring::trb::transfer::{Direction, StatusStage, TransferType};

use crate::error::OldPciResult;
use crate::xhc::device_manager::control_pipe::request::Request;
use crate::xhc::device_manager::control_pipe::{
    make_data_stage, make_setup_stage, ControlPipeTransfer,
};
use crate::xhc::device_manager::device_context_index::DeviceContextIndex;
use crate::xhc::registers::traits::doorbell_registers_accessible::DoorbellRegistersAccessible;
use crate::xhc::transfer::transfer_ring::TransferRing;

pub struct ControlOut<T>
where
    T: DoorbellRegistersAccessible,
{
    slot_id: u8,
    device_context_index: DeviceContextIndex,
    doorbell: Rc<RefCell<T>>,
    transfer_ring: Rc<RefCell<TransferRing>>,
}

impl<T> ControlOut<T>
where
    T: DoorbellRegistersAccessible,
{
    pub fn new(
        slot_id: u8,
        device_context_index: DeviceContextIndex,
        doorbell: &Rc<RefCell<T>>,
        transfer_ring: &Rc<RefCell<TransferRing>>,
    ) -> ControlOut<T> {
        Self {
            slot_id,
            device_context_index,
            doorbell: Rc::clone(doorbell),
            transfer_ring: Rc::clone(transfer_ring),
        }
    }

    fn notify(&mut self) -> OldPciResult {
        self.doorbell
            .borrow_mut()
            .notify_at(
                self.slot_id as usize,
                self.device_context_index
                    .as_u8(),
                0,
            )
    }

    fn push(&mut self, trb_buff: [u32; 4]) -> OldPciResult {
        self.transfer_ring
            .borrow_mut()
            .push(trb_buff)
    }
}

impl<T> ControlPipeTransfer for ControlOut<T>
where
    T: DoorbellRegistersAccessible,
{
    fn no_data(&mut self, request: Request) -> OldPciResult {
        let setup_stage = make_setup_stage(request.setup_stage(), TransferType::No);

        self.push(setup_stage.into_raw())?;

        let mut status = new_status_stage_with_direction();
        status.set_interrupt_on_completion();

        self.push(status.into_raw())?;
        self.notify()
    }

    fn with_data(&mut self, request: Request, data_buff_addr: u64, len: u32) -> OldPciResult {
        let setup = make_setup_stage(request.setup_stage(), TransferType::Out);
        self.push(setup.into_raw())?;

        let mut data_stage = make_data_stage(data_buff_addr, len, Direction::Out);
        data_stage.set_interrupt_on_completion();
        data_stage.set_interrupt_on_short_packet();

        self.push(data_stage.into_raw())?;

        self.push(new_status_stage_with_direction().into_raw())?;
        self.notify()
    }
}

fn new_status_stage_with_direction() -> StatusStage {
    let mut status = StatusStage::new();
    status.set_direction();
    status
}
