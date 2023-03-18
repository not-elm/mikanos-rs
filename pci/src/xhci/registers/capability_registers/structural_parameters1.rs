use macros::VolatileBits;

use crate::xhci::registers::memory_mapped_addr::MemoryMappedAddr;

pub mod number_of_device_slots;

#[derive(VolatileBits)]
#[volatile_type(u32)]
pub struct StructuralParameters1(usize);

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct StructuralParameters1Offset(usize);

impl StructuralParameters1Offset {
    pub fn new(mmio_addr: MemoryMappedAddr) -> Self {
        Self(mmio_addr.addr() + 0x04)
    }

    fn addr(&self) -> usize {
        self.0
    }
}
