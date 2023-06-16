use x86_64::structures::idt::InterruptStackFrame;

use kernel_lib::apic::LocalApicRegisters;

pub struct Timer {
    interval: Option<usize>,
    tick: usize,
}


impl Timer {
    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            interval: None,
            tick: 0,
        }
    }


    pub fn set(&mut self, interval: usize) {
        self.interval = Some(interval);
    }


    pub fn tick(&mut self) -> bool {
        if let Some(interval) = self.interval {
            self.tick += 1;
            if interval <= self.tick {
                self.tick = 0;
                return true;
            }

            false
        } else {
            false
        }
    }
}


pub static mut TIMER: Timer = Timer::new();

pub extern "x86-interrupt" fn interrupt_timer_handler(_stack_frame: InterruptStackFrame) {
    let is_interval = unsafe { TIMER.tick() };

    LocalApicRegisters::default()
        .end_of_interrupt()
        .notify();
    if is_interval {
        crate::switch();
    }
}
