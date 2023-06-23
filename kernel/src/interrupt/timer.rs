use core::sync::atomic::AtomicUsize;
use core::sync::atomic::Ordering::Relaxed;

use spin::Mutex;
use x86_64::structures::idt::InterruptStackFrame;

use kernel_lib::apic::LocalApicRegisters;

use crate::task::TASK_MANAGER;

pub struct Timer {
    interval: Mutex<Option<usize>>,
    tick: AtomicUsize,
}


impl Timer {
    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            interval: Mutex::new(None),
            tick: AtomicUsize::new(0),
        }
    }


    pub fn set(&self, interval: usize) {
        *self.interval.lock() = Some(interval);
    }


    pub fn tick(&self) -> bool {
        if let Some(interval) = *self.interval.lock() {
            let next_tick = self.tick.load(Relaxed) + 1;
            self.tick.store(next_tick, Relaxed);
            if interval <= next_tick {
                self.tick.store(0, Relaxed);
                return true;
            }

            false
        } else {
            false
        }
    }
}


pub static TIMER: Timer = Timer::new();

pub extern "x86-interrupt" fn interrupt_timer_handler(_stack_frame: InterruptStackFrame) {
    let is_interval = TIMER.tick();

    LocalApicRegisters::default()
        .end_of_interrupt()
        .notify();

    if is_interval {
        unsafe {
            TASK_MANAGER.switch_task();
        }
    }
}
