use alloc::boxed::Box;
use alloc::collections::VecDeque;
use alloc::vec;
use core::cell::OnceCell;
use core::fmt::{Debug, Formatter};
use core::sync::atomic::{AtomicU8, Ordering};

use message::TaskMessage;

use crate::context::arch::x86_64::Context;
use crate::error::KernelResult;
use crate::interrupt;
use crate::task::list::TaskList;
use crate::task::priority_level::PriorityLevel;
use crate::task::status::Status;
use crate::task::status::Status::{Pending, Running, Sleep};

mod list;
pub mod message;
pub mod priority_level;
mod status;
mod switch;


pub static mut TASK_MANAGER: PreemptiveTaskManager = PreemptiveTaskManager::new();


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


    pub fn new_task(&mut self, priority_level: PriorityLevel, rip: u64, rsi: u64) {
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
                .switch_task()
        })
    }


    #[inline(always)]
    pub fn switch_ignore_priority(&mut self) -> KernelResult {
        interrupt::asm::without_interrupt(|| {
            self.task_manager
                .get_mut()
                .unwrap()
                .switch_ignore_priority()
        })
    }
}


#[derive(Default, Debug)]
pub struct TaskManager {
    tasks: TaskList,
}


impl TaskManager {
    #[inline(always)]
    pub fn new() -> Self {
        let mut tasks = TaskList::new();
        tasks.push(Task::new_main());
        Self { tasks }
    }


    pub fn send_message_at(&mut self, task_id: u64, message: TaskMessage) -> KernelResult {
        let task = self.tasks.find_mut(task_id)?;

        if task.status().is_sleep() {
            task.store_status(Pending);
        }

        task.send_message(message);

        Ok(())
    }


    pub fn receive_message_at(&mut self, task_id: u64) -> Option<TaskMessage> {
        self.tasks
            .find_mut(task_id)
            .unwrap()
            .receive_message()
    }


    pub fn new_task(&mut self, priority_level: PriorityLevel) -> &mut Task {
        let task = self.create_task(priority_level);
        let id = task.id;
        self.tasks.push(task);

        self.tasks
            .find_mut(id)
            .unwrap()
    }


    pub fn wakeup_at(&mut self, task_id: u64) -> KernelResult {
        self.tasks.wakeup_at(task_id)
    }


    pub fn sleep_at(&mut self, task_id: u64) -> KernelResult {
        self.tasks.sleep_at(task_id)
    }


    pub fn switch_task(&mut self) -> KernelResult {
        self.tasks
            .next_switch_command()?
            .switch_if_need(Pending);

        Ok(())
    }


    pub fn switch_ignore_priority(&mut self) -> KernelResult {
        self.tasks
            .next_switch_command()?
            .switch_and_pending();

        Ok(())
    }


    #[inline]
    fn create_task(&self, priority_level: PriorityLevel) -> Task {
        Task::new(self.tasks.len() as u64, priority_level)
    }
}


pub struct Task {
    id: u64,
    priority_level: PriorityLevel,
    context: Context,
    stack: Box<[u8]>,
    messages: VecDeque<TaskMessage>,
    status: AtomicU8,
}


impl Task {
    pub fn new_main() -> Self {
        Self {
            id: 0,
            priority_level: PriorityLevel::new(3),
            context: Context::uninit(),
            stack: vec![0; 65_536].into_boxed_slice(),
            messages: VecDeque::new(),
            status: AtomicU8::new(Running as u8),
        }
    }


    pub fn new(id: u64, priority_level: PriorityLevel) -> Self {
        Self {
            id,
            priority_level,
            context: Context::uninit(),
            stack: vec![0; 65_536].into_boxed_slice(),
            messages: VecDeque::new(),
            status: AtomicU8::new(Pending as u8),
        }
    }


    #[inline(always)]
    pub fn store_status(&self, status: Status) {
        interrupt::asm::without_interrupt(|| {
            self.status
                .store(status as u8, Ordering::Relaxed);
        });
    }


    #[inline(always)]
    pub fn status(&self) -> Status {
        match self
            .status
            .load(Ordering::Relaxed)
        {
            0 => Sleep,
            1 => Pending,
            2 => Running,
            _ => panic!("Invalid Status"),
        }
    }

    #[inline(always)]
    pub fn switch_to(&self, next: &Task) {
        self.context
            .switch_to(&next.context);
    }


    pub fn init_context(&mut self, rip: u64, rsi: u64) {
        let task_end = self.stack.as_ptr_range().end as u64;
        let rsp = (task_end & !0xF) - 8;
        self.context
            .init_context(rip, self.id, rsi, rsp);
    }


    pub fn receive_message(&mut self) -> Option<TaskMessage> {
        self.messages.pop_front()
    }


    pub fn send_message(&mut self, message: TaskMessage) {
        self.messages
            .push_back(message);
    }
}


impl Debug for Task {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Task")
            .field("id", &self.id)
            .field("priority_level", &self.priority_level)
            .finish()
    }
}

#[cfg(test)]
mod tests {}
