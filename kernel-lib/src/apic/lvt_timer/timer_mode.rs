use volatile_bits::{volatile_bits, VolatileBitsWritable};

use crate::apic::lvt_timer::LvtTimerAddr;

#[volatile_bits(
offset = 17,
bits = 3,
type = u32
)]
pub struct TimerModeField(LvtTimerAddr);


impl TimerModeField {
    pub fn update_timer_mode(&self, timer_mode: TimerMode) {
        self.write_volatile(timer_mode as u32)
            .unwrap();
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
    use crate::apic::lvt_timer::LvtTimerAddr;

    #[test]
    fn it_update_timer_mode() {
        let buff = [0u32; 1];

        let timer_mode = TimerModeField::from(LvtTimerAddr::from(buff.as_ptr() as u64));
        timer_mode.update_timer_mode(Periodic);
        assert!(((buff[0] >> 17) & 0b1).is_true())
    }
}
