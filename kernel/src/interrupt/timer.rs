use x86_64::structures::idt::InterruptStackFrame;

use common_lib::queue::queueing::Queueing;
use kernel_lib::apic::LocalApicRegisters;
use kernel_lib::serial_println;

use crate::interrupt::mouse::INTERRUPT_QUEUE;
use crate::interrupt::InterruptMessage;

struct Timer {
    interval: usize,
    tick: usize,
}


impl Timer {
    #[inline(always)]
    pub const fn new(interval: usize) -> Self {
        Self { interval, tick: 0 }
    }


    pub fn tick(&mut self) -> bool {
        self.tick += 1;
        if self.interval <= self.tick {
            self.tick = 0;
            return true;
        }

        false
    }
}


static mut TIMER: Timer = Timer::new(2);

pub extern "x86-interrupt" fn interrupt_timer_handler(_stack_frame: InterruptStackFrame) {
    serial_println!("TICK");
    let is_interval = unsafe { TIMER.tick() };

    LocalApicRegisters::default()
        .end_of_interrupt()
        .notify();
    if is_interval {
        crate::switch();
    }
}
