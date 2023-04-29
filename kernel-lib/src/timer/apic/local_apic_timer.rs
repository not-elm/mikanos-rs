use crate::apic::device_config::LocalApicTimerDivide;
use crate::apic::lvt_timer::timer_mode::TimerMode::OneShot;
use crate::apic::LocalApicRegisters;
use crate::timer::apic::ApicTimer;
use crate::VolatileAccessible;

#[derive(Debug)]
pub struct OneShotLocalApicTimer {
    local_apic_registers: LocalApicRegisters,
}


impl OneShotLocalApicTimer {
    pub fn new() -> Self {
        let local_apic_registers = LocalApicRegisters::default();


        Self {
            local_apic_registers,
        }
    }


    fn local_apic_registers(&self) -> &LocalApicRegisters {
        &self.local_apic_registers
    }
}


impl ApicTimer for OneShotLocalApicTimer {
    fn start(&mut self, divide: LocalApicTimerDivide) {
        self.local_apic_registers
            .divide_config()
            .update_divide(divide);


        let lvt_timer = self
            .local_apic_registers
            .lvt_timer();

        lvt_timer
            .timer_mode()
            .update_timer_mode(OneShot);

        lvt_timer
            .mask()
            .write_flag_volatile(true);

        self.local_apic_registers
            .initial_count()
            .write_volatile(u32::MAX);
    }


    fn elapsed(&self) -> u32 {
        let init = self
            .local_apic_registers
            .initial_count()
            .read_volatile();

        let current = self
            .local_apic_registers
            .current_count()
            .read_volatile();

        init - current
    }


    fn stop(&mut self) {
        self.local_apic_registers
            .initial_count()
            .write_volatile(0);
    }
}


impl Default for OneShotLocalApicTimer {
    fn default() -> Self {
        Self::new()
    }
}
