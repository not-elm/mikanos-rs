use macros::VolatileBits;

use crate::xhci::registers::operational_registers::operation_registers_offset::OperationRegistersOffset;

#[derive(VolatileBits)]
#[offset(1)]
#[bits(1)]
pub struct HostControllerReset(usize);

impl HostControllerReset {
    pub fn new(offset: OperationRegistersOffset) -> Self {
        Self::new_uncheck(offset.offset())
    }

    pub fn reset(&self) {
        self.write_flag_volatile(true);
        while self.read_flag_volatile() {}
    }
}
