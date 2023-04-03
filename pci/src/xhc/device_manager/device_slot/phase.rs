use alloc::boxed::Box;

use crate::error::PciResult;
use xhci::ring::trb::event::TransferEvent;

use crate::xhc::transfer::event::event_trb::TargetEvent;

pub trait Phase {
    fn on_transfer_event_received(
        self,
        transfer_event: TransferEvent,
        target_event: TargetEvent,
    ) -> PciResult<Box<dyn Phase>>;
}
