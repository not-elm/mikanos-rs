use crate::apic::device_config::LocalApicTimerDivide;
use crate::timer::apic::ApicTimer;

#[derive(Default)]
pub struct MockApicTimer {
    time: u32,
}


impl MockApicTimer {
    pub fn set_time(&mut self, time: u32) {
        self.time = time;
    }
}


impl ApicTimer for MockApicTimer {
    fn start(&mut self, _: LocalApicTimerDivide) {}

    fn elapsed(&self) -> u32 {
        self.time
    }

    fn stop(&mut self) {}
}
