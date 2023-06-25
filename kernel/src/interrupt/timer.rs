use core::cell::OnceCell;
use core::sync::atomic::AtomicUsize;
use core::sync::atomic::Ordering::Relaxed;

use spin::Mutex;
use x86_64::structures::idt::InterruptStackFrame;

use kernel_lib::apic::LocalApicRegisters;
use kernel_lib::error::KernelResult;
use kernel_lib::interrupt;
use kernel_lib::interrupt::interrupt_message::TaskMessage;
use kernel_lib::task::priority_level::PriorityLevel;
use kernel_lib::task::TaskManager;

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
            let next_tick = self
                .tick
                .fetch_add(1, Relaxed);

            if interval <= next_tick {
                self.reset();
                return true;
            }

            false
        } else {
            false
        }
    }


    #[inline(always)]
    pub fn reset(&self) {
        self.tick.store(0, Relaxed);
    }
}

pub struct PreemptiveTaskManager {
    timer: Timer,
    task_manager: OnceCell<TaskManager>,
}


impl PreemptiveTaskManager {
    pub const fn new() -> Self {
        Self {
            timer: Timer::new(),
            task_manager: OnceCell::new(),
        }
    }


    #[inline(always)]
    pub fn init(&self) {
        self.task_manager
            .set(TaskManager::new())
            .unwrap();
    }


    #[inline(always)]
    pub fn set_interval(&self, interval: usize) {
        self.timer.set(interval);
    }


    pub fn send_message_at(&mut self, task_id: u64, message: TaskMessage) -> KernelResult {
        interrupt::asm::without_interrupt(|| {
            self.task_manager
                .get_mut()
                .unwrap()
                .send_message_at(task_id, message)
        })
    }


    pub fn receive_message_at(&mut self, task_id: u64) -> Option<TaskMessage> {
        self.task_manager
            .get_mut()?
            .receive_message_at(task_id)
    }


    pub unsafe fn new_task(&mut self, priority_level: PriorityLevel, rip: u64, rsi: u64) {
        self.task_manager
            .get_mut()
            .unwrap()
            .new_task(priority_level)
            .init_context(rip, rsi)
    }


    #[inline(always)]
    pub fn sleep_at(&mut self, task_id: u64) -> KernelResult {
        self.task_manager
            .get_mut()
            .unwrap()
            .sleep_at(task_id)
    }


    #[inline(always)]
    pub fn wakeup_at(&mut self, task_id: u64) -> KernelResult {
        self.task_manager
            .get_mut()
            .unwrap()
            .wakeup_at(task_id)
    }


    pub fn switch_if_timeout(&mut self) -> KernelResult {
        if self.timer.tick() {
            self.task_manager
                .get_mut()
                .unwrap()
                .switch_task()?;
        }

        Ok(())
    }


    #[inline(always)]
    pub fn switch(&mut self) -> KernelResult {
        interrupt::asm::without_interrupt(|| {
            self.timer.reset();

            self.task_manager
                .get_mut()
                .unwrap()
                .switch_ignore_priority()
        })
    }
}


pub static mut TASK_MANAGER: PreemptiveTaskManager = PreemptiveTaskManager::new();

pub extern "x86-interrupt" fn interrupt_timer_handler(_stack_frame: InterruptStackFrame) {
    LocalApicRegisters::default()
        .end_of_interrupt()
        .notify();

    unsafe {
        TASK_MANAGER
            .switch_if_timeout()
            .unwrap();
    }
}
