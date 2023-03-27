use macros::Address;

use crate::error::PciResult;
use crate::xhc::transfer::event::event_ring_segment_table::ring_segment_addr_entry::EventRingAddressEntry;
use crate::xhc::transfer::event::event_ring_segment_table::ring_segment_size::RingSegmentSize;
use crate::xhc::transfer::event::event_ring_segment_table::ring_segment_table_field::RingSegmentTableField;
use crate::VolatileAccessible;

mod ring_segment_addr_entry;
mod ring_segment_size;
mod ring_segment_table_field;

#[derive(Debug)]
pub struct EventRingSegmentTable {}

impl EventRingSegmentTable {
    pub fn new(
        event_ring_segment_table_addr: u64,
        event_ring_segment_addr: u64,
        ring_segment_size: usize,
    ) -> PciResult<Self> {
        let addr = SegmentTableAddr::new(event_ring_segment_table_addr as usize);
        EventRingAddressEntry::new(addr).write_volatile(event_ring_segment_addr);
        RingSegmentSize::new(addr).write_volatile(ring_segment_size as u32);

        Ok(Self {})
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Address)]
#[repr(transparent)]
pub(crate) struct SegmentTableAddr(usize);
