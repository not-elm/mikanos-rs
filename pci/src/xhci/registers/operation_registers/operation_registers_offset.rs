use crate::xhci::registers::capability_registers::capability_length::CapabilityLength;

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct OperationRegistersOffset(usize);

impl OperationRegistersOffset {
    pub fn new(mmio_base_addr: usize, cap_length: CapabilityLength) -> Self {
        Self(mmio_base_addr + cap_length.read_volatile() as usize)
    }

    pub fn offset(&self) -> usize {
        self.0
    }
}
