use crate::apic::device_config::LocalApicTimerDivide;

pub mod local_apic_timer;
pub mod timeout;


pub trait ApicTimer {
    fn start(&mut self, initial_count: u32, divide: LocalApicTimerDivide);


    fn elapsed(&self) -> u32;


    fn stop(&mut self);
}
