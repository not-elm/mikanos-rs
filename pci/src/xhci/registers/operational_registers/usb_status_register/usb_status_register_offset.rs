use crate::xhci::registers::operational_registers::operation_registers_offset::OperationRegistersOffset;
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct UsbStatusRegisterOffset(usize);

impl UsbStatusRegisterOffset {
    pub fn new(operation_registers_offset: OperationRegistersOffset) -> Self {
        UsbStatusRegisterOffset(operation_registers_offset.offset() + 0x04)
    }

    pub fn offset(&self) -> usize {
        self.0
    }
}
