use alloc::string::ToString;
use kernel_lib::volatile_bits::{volatile_address, VolatileBitsWritable};

use crate::error::{PciError, PciResult};
use crate::xhc::transfer::event::event_ring_segment_table::ring_segment_addr_entry::EventRingAddressEntry;
use crate::xhc::transfer::event::event_ring_segment_table::ring_segment_size::RingSegmentSize;

mod ring_segment_addr_entry;
mod ring_segment_size;

#[derive(Debug)]
pub struct EventRingSegmentTable {}

impl EventRingSegmentTable {
    pub fn new(
        event_ring_segment_table_addr: u64,
        event_ring_segment_addr: u64,
        ring_segment_size: usize,
    ) -> PciResult<Self> {
        let addr = SegmentTableAddr::from(event_ring_segment_table_addr);
        EventRingAddressEntry::from(addr)
            .write_volatile(event_ring_segment_addr)
            .map_err(|_| PciError::new("Failed write Event Ring Segment Addr".to_string()))?;

        RingSegmentSize::from(addr)
            .write_volatile(ring_segment_size as u32)
            .map_err(|_| PciError::new("Failed write Ring Segment Size".to_string()))?;

        Ok(Self {})
    }
}

#[volatile_address]
pub struct SegmentTableAddr(u64);
