use macros::Address;

use crate::error::{AllocateReason, PciError, PciResult};
use crate::xhc::allocator::memory_allocatable::MemoryAllocatable;
use crate::xhc::transfer::request_block::TRB_SIZE;

#[derive(Debug)]
pub struct Segment {
    segment_base_addr: RingSegmentsBaseAddr,
}

impl Segment {
    pub fn new(trb_buffer_len: usize, allocator: &mut impl MemoryAllocatable) -> PciResult<Self> {
        Ok(Self {
            segment_base_addr: allocate_segment(trb_buffer_len, allocator)?,
        })
    }

    pub fn base_addr(&self) -> &RingSegmentsBaseAddr {
        &self.segment_base_addr
    }
}

fn allocate_segment(
    segment_size: usize,
    allocator: &mut impl MemoryAllocatable,
) -> PciResult<RingSegmentsBaseAddr> {
    let segment_base_addr =
        unsafe { allocator.allocate_with_align(TRB_SIZE * segment_size, 64, 64 * 1024) }
            .ok_or(PciError::FailedAllocate(AllocateReason::NotEnoughMemory))?
            .address()?;
    Ok(RingSegmentsBaseAddr::new(segment_base_addr))
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Address)]
#[repr(transparent)]
pub struct RingSegmentsBaseAddr(usize);
