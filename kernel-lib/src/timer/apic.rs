use crate::apic::device_config::LocalApicTimerDivide;

pub mod local_apic_timer;
pub mod timeout;


pub trait ApicTimer {
    fn start(&mut self, divide: LocalApicTimerDivide);


    fn elapsed(&self) -> u32;


    fn stop(&mut self);
}
