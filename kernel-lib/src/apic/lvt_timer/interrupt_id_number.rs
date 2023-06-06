use crate::apic::lvt_timer::LvtTimerAddr;
use crate::volatile_bits::volatile_bits;

#[volatile_bits(
type = u32,
bits = 16
)]
#[derive(Debug, Copy, Clone)]
pub struct InterruptIdNumber(LvtTimerAddr);
