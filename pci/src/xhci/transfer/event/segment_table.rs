use crate::error::PciResult;
use macros::Address;

use crate::xhci::transfer::event::segment::RingSegmentsBaseAddr;
use crate::xhci::transfer::event::segment_table::ring_segment_addr_entry::RingSegmentsBaseAddrEntry;
use crate::xhci::transfer::event::segment_table::ring_segment_table_field::RingSegmentTableField;

use crate::xhci::allocator::memory_allocatable::MemoryAllocatable;

mod ring_segment_addr_entry;
mod ring_segment_table_field;

#[repr(align(64))]
#[derive(Debug)]
pub struct SegmentTable {
    ring_segments_base_addr_entry: RingSegmentsBaseAddrEntry,
}

impl SegmentTable {
    pub fn new(
        segment_table_addr: SegmentTableAddr,
        allocator: &mut impl MemoryAllocatable,
    ) -> PciResult<Self> {
        let ring_segments_base_addr_entry = RingSegmentsBaseAddrEntry::new(segment_table_addr);

        // let ring_segment = allocator.allocate_with_align(64, 64 * 1024);
        todo!();
        Ok(Self {
            ring_segments_base_addr_entry,
        })
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Address)]
#[repr(transparent)]
pub struct SegmentTableAddr(usize);
