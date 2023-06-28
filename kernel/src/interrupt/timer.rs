use core::cell::OnceCell;
use core::sync::atomic::AtomicUsize;
use core::sync::atomic::Ordering::Relaxed;

use x86_64::structures::idt::InterruptStackFrame;

use kernel_lib::apic::LocalApicRegisters;
use kernel_lib::error::KernelResult;
use kernel_lib::interrupt;
use kernel_lib::interrupt::interrupt_message::TaskMessage;
use kernel_lib::task::priority_level::PriorityLevel;
use kernel_lib::task::TaskManager;
use kernel_lib::timer::TIME_HANDLE_MANAGER;

pub struct PreemptiveTaskManager {
    task_manager: OnceCell<TaskManager>,
}


impl PreemptiveTaskManager {
    pub const fn new() -> Self {
        Self {
            task_manager: OnceCell::new(),
        }
    }


    #[inline(always)]
    pub fn init(&self) {
        self.task_manager
            .set(TaskManager::new())
            .unwrap();
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


    #[inline(always)]
    pub fn switch(&mut self) -> KernelResult {
        interrupt::asm::without_interrupt(|| {
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

    TIME_HANDLE_MANAGER.tick();
}
