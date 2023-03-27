use crate::error::PciResult;
use crate::xhc::allocator::memory_allocatable::MemoryAllocatable;
use crate::xhc::transfer::event::segment::RingSegmentsBaseAddr;
use crate::xhc::transfer::event::segment_table::ring_segment_addr_entry::EventRingAddressEntry;
use crate::xhc::transfer::ring::RingBase;
use crate::VolatileAccessible;

#[derive(Debug)]
pub struct EventRingTable {}

impl EventRingTable {
    pub fn new(command_ring_table_addr: u64, command_ring_addr: u64) -> PciResult<Self> {
        EventRingAddressEntry::new_uncheck(command_ring_table_addr as usize)
            .update_ring_segment_addr(&RingSegmentsBaseAddr::new(command_ring_addr as usize))?;
        Ok(Self {})
    }
}
