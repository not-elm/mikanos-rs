use crate::xhci::registers::capability_registers::structural_parameters1::number_of_device_slots::NumberOfDeviceSlots;
use crate::xhci::registers::memory_mapped_addr::MemoryMappedAddr;

pub mod number_of_device_slots;

#[derive(Debug)]
pub struct StructuralParameters1 {
    max_slots: NumberOfDeviceSlots,
}

impl StructuralParameters1 {
    pub fn new(offset: StructuralParameters1Offset) -> Self {
        Self {
            max_slots: NumberOfDeviceSlots::new(offset),
        }
    }

    pub fn max_slots(&self) -> &NumberOfDeviceSlots {
        &self.max_slots
    }
}

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
