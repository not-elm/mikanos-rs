use alloc::boxed::Box;
use alloc::collections::VecDeque;
use alloc::sync::Arc;
use alloc::vec;
use core::cell::OnceCell;

use spin::RwLock;

use crate::{kernel_bail, kernel_error};
use crate::context::arch::x86_64::Context;
use crate::error::KernelResult;
use crate::interrupt::interrupt_message::TaskMessage;
use crate::task::list::TaskList;
use crate::task::priority_level::PriorityLevel;
use crate::task::status::Status;

pub mod priority_level;
mod list;
mod status;
mod switch;


pub struct CellTaskManger(OnceCell<TaskManager>);


impl CellTaskManger {
    #[inline(always)]
    pub const fn uninit() -> Self {
        Self(OnceCell::new())
    }


    pub fn init(&self) -> Result<(), TaskManager> {
        self.0.set(TaskManager::new())
    }


    pub fn receive_message_at(&mut self, task_id: u64) -> Option<TaskMessage> {
        self.0
            .get_mut()?
            .receive_message_at(task_id)
    }


    pub fn new_task(&mut self, priority_level: PriorityLevel) -> &mut Task {
        self.0
            .get_mut()
            .unwrap()
            .new_task(priority_level)
    }


    pub fn switch_task(&mut self) {
        if let Some(manager) = self.0.get_mut() {
            manager.switch_task();
        }
    }


    pub fn send_message_at(&mut self, task_id: u64, message: TaskMessage) -> KernelResult {
        if let Some(task_manager) = self.0.get_mut() {
            return task_manager.send_message_at(task_id, message);
        }

        kernel_bail!("Not Initialized Task Manager!")
    }


    #[inline(always)]
    pub fn sleep_at(&mut self, task_id: u64) -> KernelResult {
        self.get_mut()?
            .sleep_at(task_id)
    }


    pub fn wakeup_at(&mut self, task_id: u64) -> KernelResult {
        self.get_mut()?
            .wakeup_at(task_id)
    }


    fn get_mut(&mut self) -> KernelResult<&mut TaskManager> {
        self.0
            .get_mut()
            .ok_or(kernel_error!("Uninitialized CellTaskManager"))
    }
}


unsafe impl Sync for CellTaskManger {}


#[derive(Default, Debug)]
pub struct TaskManager {
    tasks: TaskList,
}


impl TaskManager {
    #[inline(always)]
    pub fn new() -> Self {
        let mut tasks = TaskList::new();
        tasks.push(Task::new(0, PriorityLevel::new(3)));
        Self {
            tasks
        }
    }


    pub fn send_message_at(&mut self, task_id: u64, message: TaskMessage) -> KernelResult {
        self.tasks
            .find_mut(task_id)?

            .send_message(message);

        Ok(())
    }


    pub fn receive_message_at(&mut self, task_id: u64) -> Option<TaskMessage> {
        self.tasks
            .find_mut(task_id)
            .ok()?
            .receive_message()
    }


    pub fn new_task(&mut self, priority_level: PriorityLevel) -> &mut Task {
        let task = self.create_task(priority_level);
        let id = task.id;
        self.tasks
            .push(task);

        self.tasks
            .find_mut(id)
            .unwrap()
    }


    pub fn wakeup_at(&mut self, task_id: u64) -> KernelResult {
        self.tasks
            .wakeup_at(task_id)
    }


    pub fn sleep_at(&mut self, task_id: u64) -> KernelResult {
        self.tasks
            .sleep_at(task_id)
    }


    pub fn switch_task(&mut self) -> KernelResult {
        self
            .tasks
            .next_switch_command()?
            .switch_and_pending();

        Ok(())
    }


    #[inline]
    fn create_task(&self, priority_level: PriorityLevel) -> Task {
        Task::new(self.tasks.len() as u64, priority_level)
    }
}


#[derive(Debug)]
pub struct Task {
    id: u64,
    priority_level: PriorityLevel,
    context: Context,
    stack: Box<[u8]>,
    messages: VecDeque<TaskMessage>,
    status: Status,
}


impl Task {
    pub fn new(id: u64, priority_level: PriorityLevel) -> Self {
        Self {
            id,
            priority_level,
            context: Context::uninit(),
            stack: vec![0; 65_536].into_boxed_slice(),
            messages: VecDeque::new(),
            status: Status::Pending,
        }
    }


    #[inline(always)]
    pub fn switch_to(&self, next: &Task) {
        self.context
            .switch_to(&next.context);
    }


    pub unsafe fn init_context(&mut self, rip: u64, rsi: u64) {
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

#[cfg(test)]
mod tests {}
