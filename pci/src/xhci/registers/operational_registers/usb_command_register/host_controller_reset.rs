use crate::xhci::registers::operational_registers::operation_registers_offset::OperationalRegistersOffset;
use core::marker::PhantomData;
use macros::VolatileBits;

#[derive(VolatileBits)]
#[offset(1)]
#[bits(1)]
pub struct HostControllerReset(usize, PhantomData<OperationalRegistersOffset>);

impl HostControllerReset {
    pub fn reset(&self) {
        self.write_flag_volatile(true);
        while self.read_flag_volatile() {}
    }
}
