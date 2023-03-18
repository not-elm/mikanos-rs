use macros::VolatileBits;

use crate::xhci::registers::operational_registers::operation_registers_offset::OperationalRegistersOffset;

#[derive(VolatileBits)]
pub struct HostSystemErrorEnable(usize);

impl HostSystemErrorEnable {
    pub fn new(offset: OperationalRegistersOffset) -> Self {
        Self::new_uncheck(offset.offset() + 3)
    }
}
