use macros::VolatileBits;

use crate::xhci::registers::operation_registers::operation_registers_offset::OperationRegistersOffset;

#[derive(VolatileBits)]
pub struct HostSystemErrorEnable(usize);

impl HostSystemErrorEnable {
    pub fn new(offset: OperationRegistersOffset) -> Self {
        Self::new_uncheck(offset.offset() + 3)
    }
}
