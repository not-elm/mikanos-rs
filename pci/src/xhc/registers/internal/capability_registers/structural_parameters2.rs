use crate::error::PciResult;
use crate::xhc::registers::internal::capability_registers::structural_parameters2::event_ring_segment_table_max::EventRingSegmentTableMax;
use crate::xhc::registers::internal::memory_mapped_addr::MemoryMappedAddr;

pub mod event_ring_segment_table_max;
pub mod structural_parameters2_field;

/// # Address
///
/// MemoryMappedAddress + 0x08 Bytes
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct StructuralParameters2Offset(usize);

impl StructuralParameters2Offset {
    pub fn new(mmio_addr: MemoryMappedAddr) -> Self {
        Self(mmio_addr.addr() + 0x08)
    }
    pub fn offset(&self) -> usize {
        self.0
    }
}

/// # Address
///
/// MemoryMappedAddress + 0x08 Bytes
#[derive(Debug)]
pub struct StructuralParameters2 {
    erst_max: EventRingSegmentTableMax,
}

impl StructuralParameters2 {
    pub fn new(offset: StructuralParameters2Offset) -> PciResult<Self> {
        Ok(Self {
            erst_max: EventRingSegmentTableMax::new_with_check_size(offset)?,
        })
    }

    pub fn erst_max(&self) -> &EventRingSegmentTableMax {
        &self.erst_max
    }
}
