use crate::apic::lvt_timer::lvt_timer_field::LvtTimerField;
use crate::apic::lvt_timer::mask::Mask;
use crate::apic::lvt_timer::timer_mode::TimerModeField;
use crate::apic::LocalApicRegistersAddr;

pub mod lvt_timer_field;
pub mod mask;
pub mod timer_mode;


#[derive(Debug)]
pub struct LvtTimer {
    mask: Mask,
    timer_mode: TimerModeField,
}


impl LvtTimer {
    pub fn new(lvt_timer_addr: LvtTimerAddr) -> Self {
        Self {
            mask: Mask::new(lvt_timer_addr),
            timer_mode: TimerModeField::new(lvt_timer_addr),
        }
    }


    pub fn mask(&self) -> &Mask {
        &self.mask
    }


    pub fn timer_mode(&self) -> &TimerModeField {
        &self.timer_mode
    }
}


#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct LvtTimerAddr(usize);


impl LvtTimerAddr {
    pub fn new(addr: LocalApicRegistersAddr) -> Self {
        Self(addr.addr() + 0x320)
    }


    pub fn raw(&self) -> usize {
        self.0
    }
}


impl Default for LvtTimerAddr {
    fn default() -> Self {
        Self::new(LocalApicRegistersAddr::default())
    }
}
