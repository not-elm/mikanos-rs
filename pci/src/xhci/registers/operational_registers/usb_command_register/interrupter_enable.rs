use core::marker::PhantomData;

use macros::VolatileBits;

use crate::xhci::registers::operational_registers::operation_registers_offset::OperationalRegistersOffset;

#[derive(VolatileBits)]
#[offset(2)]
#[bits(1)]
pub struct InterrupterEnable(usize, PhantomData<OperationalRegistersOffset>);
