use crate::apic::lvt_timer::LvtTimerAddr;
use crate::VolatileAccessible;

pub trait LvtTimerField<VolatileType> {
    fn new(lvt_timer_addr: LvtTimerAddr) -> Self;
}


impl<Volatile, VolatileType> LvtTimerField<VolatileType> for Volatile
where
    Volatile: VolatileAccessible<VolatileType, usize, LvtTimerAddr>,
{
    fn new(lvt_timer_addr: LvtTimerAddr) -> Self {
        Self::new_uncheck(lvt_timer_addr.raw())
    }
}
