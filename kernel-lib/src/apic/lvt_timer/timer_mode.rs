use core::marker::PhantomData;

use macros::VolatileBits;

use crate::apic::lvt_timer::LvtTimerAddr;

#[derive(VolatileBits)]
#[bits(2)]
#[offset_bit(17)]
#[volatile_type(u8)]
pub struct TimerModeField(usize, PhantomData<LvtTimerAddr>);


impl TimerModeField {
    pub fn update_timer_mode(&self, timer_mode: TimerMode) {
        self.write_volatile(timer_mode as u8);
    }
}


#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
#[repr(u8)]
pub enum TimerMode {
    OneShot = 0b00,
    Periodic = 0b01,
    TscDeadline = 0b10,
}


#[cfg(test)]
mod tests {
    use common_lib::nums::FlagConvertible;

    use crate::apic::lvt_timer::timer_mode::TimerMode::Periodic;
    use crate::apic::lvt_timer::timer_mode::TimerModeField;
    use crate::VolatileAccessible;

    #[test]
    fn it_update_timer_mode() {
        let buff = [0u32; 1];

        let timer_mode = TimerModeField::new_uncheck(buff.as_ptr().addr());
        timer_mode.update_timer_mode(Periodic);
        assert!(((buff[0] >> 17) & 0b1).is_true())
    }
}
