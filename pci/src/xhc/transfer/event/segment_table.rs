use macros::Address;

use crate::error::{AllocateReason, PciError, PciResult};
use crate::xhc::allocator::memory_allocatable::MemoryAllocatable;
use crate::xhc::transfer::event::segment::{RingSegmentsBaseAddr, Segment};
use crate::xhc::transfer::event::segment_table::ring_segment_addr_entry::RingSegmentsBaseAddrEntry;
use crate::xhc::transfer::event::segment_table::ring_segment_table_field::RingSegmentTableField;

mod ring_segment_addr_entry;
mod ring_segment_table_field;

///
///
/// [Xhci Document]: 514 Page
#[derive(Debug)]
pub struct SegmentTable {
    segment_table_addr: SegmentTableAddr,
    ring_segments_base_addr_entry: RingSegmentsBaseAddrEntry,
    // 現状1つのみサポート
    #[allow(unused)]
    ring_segment: Segment,
}

impl SegmentTable {
    pub fn new(ring_segment: Segment, allocator: &mut impl MemoryAllocatable) -> PciResult<Self> {
        let segment_table_addr = allocate_segment_table(allocator)?;

        let ring_segments_base_addr_entry = RingSegmentsBaseAddrEntry::new(segment_table_addr);
        ring_segments_base_addr_entry.update_ring_segment_addr(ring_segment.base_addr())?;

        Ok(Self {
            segment_table_addr,
            ring_segments_base_addr_entry,
            ring_segment,
        })
    }

    pub fn segment_table_addr(&self) -> SegmentTableAddr {
        self.segment_table_addr
    }

    pub fn segments_base_addr(&self) -> &RingSegmentsBaseAddr {
        self.ring_segment.base_addr()
    }
}

fn allocate_segment_table(allocator: &mut impl MemoryAllocatable) -> PciResult<SegmentTableAddr> {
    const SEGMENT_TABLE_SIZE: usize = 16;

    let segment_base_addr =
        unsafe { allocator.allocate_with_align(SEGMENT_TABLE_SIZE, 64, 64 * 1024) }
            .ok_or(PciError::FailedAllocate(AllocateReason::NotEnoughMemory))?
            .address()?;
    Ok(SegmentTableAddr::new(segment_base_addr))
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Address)]
#[repr(transparent)]
pub struct SegmentTableAddr(usize);
