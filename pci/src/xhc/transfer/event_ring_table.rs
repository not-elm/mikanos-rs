use crate::error::PciResult;
use crate::xhc::transfer::event::segment::RingSegmentsBaseAddr;
use crate::xhc::transfer::event::segment_table::ring_segment_addr_entry::EventRingAddressEntry;
use crate::VolatileAccessible;

#[derive(Debug)]
pub struct EventRingTable {}

impl EventRingTable {
    pub fn new(command_ring_table_addr: u64, command_ring_addr: u64) -> PciResult<Self> {
        EventRingAddressEntry::new_uncheck(command_ring_table_addr as usize)
            .update_ring_segment_addr(&RingSegmentsBaseAddr::new(command_ring_addr as usize))?;
        let ptr = (command_ring_table_addr + 0x08) as *mut u32;
        unsafe {
            *ptr = 32;
        }
        Ok(Self {})
    }
}
