use crate::xhci::registers::operation_registers::operation_registers_offset::OperationRegistersOffset;
use macros::VolatileFlag;

#[derive(VolatileFlag)]
pub struct HostControllerReset(usize);

impl HostControllerReset {
    pub fn new(offset: OperationRegistersOffset) -> Self {
        Self::new_uncheck(offset.offset() + 1)
    }

    pub fn reset(&self) {
        self.write_volatile(true);
        while !self.read_volatile() {}
    }
}
