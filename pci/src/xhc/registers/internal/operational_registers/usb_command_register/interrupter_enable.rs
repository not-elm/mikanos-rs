use core::marker::PhantomData;

use macros::VolatileBits;

use crate::xhc::registers::internal::operational_registers::operation_registers_offset::OperationalRegistersOffset;

#[derive(VolatileBits)]
#[offset_bit(2)]
#[bits(1)]
pub struct InterrupterEnable(usize, PhantomData<OperationalRegistersOffset>);
