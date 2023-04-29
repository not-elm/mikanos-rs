use core::marker::PhantomData;

use macros::VolatileBits;

use crate::apic::lvt_timer::LvtTimerAddr;

#[derive(VolatileBits)]
#[offset_bit(16)]
#[volatile_type(u8)]
#[bits(1)]
pub struct Mask(usize, PhantomData<LvtTimerAddr>);
