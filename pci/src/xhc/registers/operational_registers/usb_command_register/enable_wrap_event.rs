use core::marker::PhantomData;

use macros::VolatileBits;

use crate::xhc::registers::operational_registers::operation_registers_offset::OperationalRegistersOffset;

#[derive(VolatileBits)]
#[offset_bit(10)]
#[bits(1)]
pub struct EnableWrapEvent(usize, PhantomData<OperationalRegistersOffset>);
