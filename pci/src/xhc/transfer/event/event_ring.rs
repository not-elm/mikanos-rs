use crate::error::PciResult;
use crate::xhc::registers::traits::interrupter_set_register_accessible::InterrupterSetRegisterAccessible;
use crate::xhc::transfer::event::event_trb::EventTrb;
use crate::xhc::transfer::transfer_ring::TransferRing;
use crate::xhc::transfer::trb_byte_size;
use crate::xhc::transfer::trb_raw_data::TrbRawData;
use kernel_lib::serial_println;

#[derive(Debug)]
pub struct EventRing {
    transfer_ring: TransferRing,
}

impl EventRing {
    pub fn new(segment_base_addr: u64, ring_size: usize) -> Self {
        Self {
            transfer_ring: TransferRing::new(segment_base_addr, ring_size, true),
        }
    }

    pub fn read_event_trb(
        &self,
        interrupter_set: &impl InterrupterSetRegisterAccessible,
    ) -> Option<EventTrb> {
        let event_ring_dequeue_pointer_addr = interrupter_set.read_event_ring_addr(0);

        let trb_raw_data =
            TrbRawData::new_unchecked(unsafe { *(event_ring_dequeue_pointer_addr as *mut u128) });

        unsafe { EventTrb::new(trb_raw_data, self.transfer_ring.cycle_bit()) }
    }

    pub fn next_dequeue_pointer(
        &mut self,
        interrupter_set: &mut impl InterrupterSetRegisterAccessible,
    ) -> PciResult {
        let dequeue_pointer_addr = interrupter_set.read_event_ring_addr(0);
        let next_addr = dequeue_pointer_addr + trb_byte_size();
        if self.transfer_ring.is_end_address(next_addr) {
            self.transfer_ring.toggle_cycle_bit();
            interrupter_set.write_event_ring_dequeue_pointer(0, self.transfer_ring.base_address())
        } else {
            interrupter_set.write_event_ring_dequeue_pointer(0, next_addr)
        }
    }
}
