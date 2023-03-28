use crate::error::PciResult;
use crate::xhc::registers::interrupter_set_register_accessible::InterrupterSetRegisterAccessible;
use crate::xhc::transfer::event::trb::EventTrb;
use crate::xhc::transfer::trb_raw_data::TrbRawData;

#[derive(Debug)]
pub struct EventRingSegment {
    cycle_bit: bool,
    segment_base_addr: u64,
    segment_end_addr: u64,
    ring_size: usize,
}

impl EventRingSegment {
    pub fn new(segment_base_addr: u64, ring_size: usize) -> Self {
        Self {
            cycle_bit: true,
            segment_base_addr,
            segment_end_addr: segment_base_addr + (core::mem::size_of::<u128>() + ring_size) as u64,
            ring_size,
        }
    }

    pub fn read_event_trb(&self, event_ring_dequeue_pointer_addr: u64) -> Option<EventTrb> {
        let ptr = event_ring_dequeue_pointer_addr as *const u128;
        if ptr.is_null() {
            return None;
        }
        let raw_data = unsafe { *(ptr) };
        let trb_raw_data = TrbRawData::new(raw_data).ok()?;
        unsafe { EventTrb::new(trb_raw_data, self.cycle_bit) }
    }

    pub fn next_dequeue_pointer(
        &mut self,
        interrupter_set: &mut impl InterrupterSetRegisterAccessible,
    ) -> PciResult {
        let dequeue_pointer_addr = interrupter_set.read_event_ring_addr(0);
        let next_addr = dequeue_pointer_addr + core::mem::size_of::<u128>() as u64;
        if next_addr < self.segment_end_addr {
            interrupter_set.write_event_ring_dequeue_pointer(0, next_addr)
        } else {
            self.cycle_bit = !self.cycle_bit;
            interrupter_set.write_event_ring_dequeue_pointer(0, self.segment_base_addr)
        }
    }
}
