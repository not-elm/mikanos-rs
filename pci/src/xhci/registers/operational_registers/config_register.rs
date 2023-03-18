use crate::xhci::registers::operational_registers::operation_registers_offset::OperationRegistersOffset;

pub mod max_device_slots_enabled;

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct ConfigRegisterOffset(usize);

impl ConfigRegisterOffset {
    pub fn new(operation_registers_offset: OperationRegistersOffset) -> Self {
        Self(operation_registers_offset.offset() + 0x38)
    }

    pub fn offset(&self) -> usize {
        self.0
    }
}
