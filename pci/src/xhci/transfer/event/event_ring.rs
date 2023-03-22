use crate::error::PciResult;
use crate::xhci::allocator::memory_allocatable::MemoryAllocatable;
use crate::xhci::transfer::event::segment::Segment;
use crate::xhci::transfer::event::segment_table::{SegmentTable, SegmentTableAddr};

#[derive(Debug)]
pub struct EventRing {
    segment_table: SegmentTable,
}

impl EventRing {
    pub fn new(trb_buffer_len: usize, allocator: &mut impl MemoryAllocatable) -> PciResult<Self> {
        let ring_segment = Segment::new(trb_buffer_len, allocator)?;
        Ok(Self {
            segment_table: SegmentTable::new(ring_segment, allocator)?,
        })
    }

    pub fn segment_table(&self) -> &SegmentTable {
        &self.segment_table
    }

    pub fn trb(&mut self) -> *const u16 {
        self.segment_table.trb()
    }
}
