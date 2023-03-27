use crate::error::PciResult;
use crate::xhc::allocator::memory_allocatable::MemoryAllocatable;
use crate::xhc::transfer::event::segment::Segment;
use crate::xhc::transfer::event::segment_table::SegmentTable;

#[derive(Debug)]
pub struct EventRingA {
    segment_table: SegmentTable,
}

impl EventRingA {
    pub fn new(trb_buffer_len: usize, allocator: &mut impl MemoryAllocatable) -> PciResult<Self> {
        let ring_segment = Segment::new(trb_buffer_len, allocator)?;
        Ok(Self {
            segment_table: SegmentTable::new(ring_segment, allocator)?,
        })
    }

    pub fn segment_table(&self) -> &SegmentTable {
        &self.segment_table
    }
}
