use core::marker::PhantomData;
use macros::VolatileBits;

use crate::xhc::registers::internal::operational_registers::operation_registers_offset::OperationalRegistersOffset;

#[derive(VolatileBits)]
#[offset_bit(1)]
#[bits(1)]
pub struct HostControllerReset(usize, PhantomData<OperationalRegistersOffset>);

impl HostControllerReset {
    pub fn reset(&self) {
        self.write_flag_volatile(true);
        while self.read_flag_volatile() {}
    }
}
