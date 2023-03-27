use crate::xhc::registers::internal::operational_registers::operation_registers_offset::OperationalRegistersOffset;
use core::marker::PhantomData;
use macros::VolatileBits;

#[derive(VolatileBits)]
pub struct RunStop(usize, PhantomData<OperationalRegistersOffset>);
