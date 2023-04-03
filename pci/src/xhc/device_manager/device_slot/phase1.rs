use crate::xhc::device_manager::device_slot::phase::Phase;
use crate::xhc::registers::traits::doorbell_registers_accessible::DoorbellRegistersAccessible;
use crate::xhc::transfer::event::event_trb::TargetEvent;
use alloc::boxed::Box;
use alloc::rc::Rc;
use core::cell::RefCell;
use xhci::ring::trb::event::TransferEvent;
use crate::error::PciResult;

pub struct Phase1<T>
where
    T: DoorbellRegistersAccessible,
{
    doorbell: Rc<RefCell<T>>,
}

impl<T> Phase1<T>
where
    T: DoorbellRegistersAccessible,
{
    pub fn new(doorbell: &Rc<RefCell<T>>) -> Phase1<T> {
        Self {
            doorbell: Rc::clone(doorbell),
        }
    }
}
impl<T> Phase for Phase1<T>
where
    T: DoorbellRegistersAccessible,
{
    fn on_transfer_event_received(
        self,
        transfer_event: TransferEvent,
        target_event: TargetEvent,
    ) -> PciResult<Box<dyn Phase>> {
        if let TargetEvent::StatusStage(status_stage)  = {

        }
    }
}
