use core::marker::PhantomData;

use crate::xhc::registers::internal::capability_registers::structural_parameters1::StructuralParameters1Offset;
use macros::VolatileBits;

#[derive(VolatileBits)]
#[volatile_type(u32)]
#[offset_bit(24)]
/// 扱えるデバイスの数
/// MaxSlots
pub struct NumberOfPorts(usize, PhantomData<StructuralParameters1Offset>);
