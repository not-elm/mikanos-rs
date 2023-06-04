use volatile_bits::volatile_bits;

use crate::apic::lvt_timer::LvtTimerAddr;

#[volatile_bits(
type = u32,
bits = 1,
offset = 16
)]
pub struct Mask(LvtTimerAddr);
