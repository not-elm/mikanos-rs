use macros::VolatileBits;

use crate::xhci::registers::operational_registers::operation_registers_offset::OperationalRegistersOffset;

#[derive(VolatileBits)]
pub struct RunStop(usize);

impl RunStop {
    pub fn new(offset: OperationalRegistersOffset) -> Self {
        Self::new_uncheck(offset.offset())
    }
}
