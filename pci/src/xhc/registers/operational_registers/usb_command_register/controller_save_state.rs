use core::marker::PhantomData;

use macros::VolatileBits;

use crate::xhc::registers::operational_registers::operation_registers_offset::OperationalRegistersOffset;

#[derive(VolatileBits)]
pub struct ControllerSaveState(usize, PhantomData<OperationalRegistersOffset>);
