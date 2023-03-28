use crate::error::PciResult;
use crate::xhc::registers::traits::doorbell_registers_accessible::DoorbellRegistersAccessible;
use crate::xhc::transfer::ring::Ring;
use crate::xhc::transfer::trb_raw_data::TrbRawData;

#[derive(Debug)]
pub struct CommandRing {
    transfer_ring: Ring,
}

impl CommandRing {
    pub fn new(ring_ptr_addr: u64, ring_size: usize) -> Self {
        Self {
            transfer_ring: Ring::new(ring_ptr_addr, ring_size, true),
        }
    }
    pub fn push_no_op(&mut self, doorbell: &mut impl DoorbellRegistersAccessible) -> PciResult {
        self.transfer_ring.push(TrbRawData::from(
            xhci::ring::trb::command::Noop::new().into_raw(),
        ))?;
        doorbell.notify_at(0, 0, 0)
    }
    pub fn push_enable_slot(
        &mut self,
        doorbell: &mut impl DoorbellRegistersAccessible,
    ) -> PciResult {
        self.transfer_ring.push(TrbRawData::from(
            xhci::ring::trb::command::EnableSlot::new().into_raw(),
        ))?;
        doorbell.notify_at(0, 0, 0)
    }
}