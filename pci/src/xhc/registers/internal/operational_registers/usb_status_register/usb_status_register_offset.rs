use crate::xhc::registers::internal::operational_registers::operation_registers_offset::OperationalRegistersOffset;

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct UsbStatusRegisterOffset(usize);

impl UsbStatusRegisterOffset {
    pub fn new(operation_registers_offset: OperationalRegistersOffset) -> Self {
        UsbStatusRegisterOffset(operation_registers_offset.offset() + 0x04)
    }

    pub fn offset(&self) -> usize {
        self.0
    }
}
