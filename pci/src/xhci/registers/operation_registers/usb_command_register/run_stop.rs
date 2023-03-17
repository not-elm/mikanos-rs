use crate::xhci::registers::operation_registers::operation_registers_offset::OperationRegistersOffset;
use macros::VolatileFlag;

#[derive(VolatileFlag)]
pub struct RunStop(usize);

impl RunStop {
    pub fn new(offset: OperationRegistersOffset) -> Self {
        Self::new_uncheck(offset.offset())
    }
}
