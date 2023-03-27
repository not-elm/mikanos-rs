use core::marker::PhantomData;

use macros::VolatileBits;

use crate::xhc::registers::internal::operational_registers::operation_registers_offset::OperationalRegistersOffset;

#[derive(VolatileBits)]
pub struct LightHostControllerReset(usize, PhantomData<OperationalRegistersOffset>);
