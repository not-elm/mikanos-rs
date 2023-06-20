use alloc::boxed::Box;
use alloc::collections::VecDeque;
use alloc::vec;
use core::cell::OnceCell;

use crate::{kernel_error, serial_println};
use crate::context::arch::x86_64::{asm_switch_context, Context};
use crate::error::{KernelError, KernelResult};

pub struct CellTaskManger(OnceCell<TaskManager>);


impl CellTaskManger {
    #[inline(always)]
    pub const fn uninit() -> Self {
        Self(OnceCell::new())
    }


    pub fn init(&self) -> Result<(), TaskManager> {
        self.0.set(TaskManager::new())
    }


    pub fn new_task(&mut self) -> &mut Task {
        self.0
            .get_mut()
            .unwrap()
            .new_task()
    }


    pub fn switch_task(&mut self) {
        if let Some(manager) = self.0.get_mut() {
            manager.switch_task();
        }
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
    available_tasks: VecDeque<Task>,
    sleep_tasks: VecDeque<Task>,
}


impl TaskManager {
    #[inline(always)]
    pub fn new() -> Self {
        let mut available_tasks = VecDeque::new();
        available_tasks.push_back(Task::new(0));
        Self {
            available_tasks,
            sleep_tasks: VecDeque::new(),
        }
    }


    pub fn new_task(&mut self) -> &mut Task {
        self.available_tasks
            .push_back(self.create_task());

        self.available_tasks
            .back_mut()
            .unwrap()
    }


    pub fn wakeup_at(&mut self, task_id: u64) -> KernelResult {
        let task = self.remove_sleep_task_at(task_id)?;
        self.available_tasks
            .push_back(task);

        Ok(())
    }


    pub fn sleep_at(&mut self, task_id: u64) -> KernelResult {
        if self.front_available()?.id == task_id {
            serial_println!("Running id = {}", task_id);
            let task = self
                .available_tasks
                .pop_front()
                .unwrap();
            self.sleep(task);
        } else {
            serial_println!("Not Running id = {}", task_id);

            let task = self.remove_available_task_at(task_id)?;
            self.sleep_tasks.push_back(task);

            let running = self.available_tasks.pop_front().unwrap();
            self.available_tasks.push_back(running);
            let running = self.available_tasks.back().unwrap();
            let next = self.available_tasks.front().unwrap();
            running.switch_to(next);
        }

        Ok(())
    }


    fn sleep(&mut self, task: Task) {
        self.sleep_tasks
            .push_back(task);
        self.sleep_tasks.back()
            .unwrap()
            .switch_to(
                self
                    .available_tasks
                    .front()
                    .unwrap()
            )
    }


    pub fn switch_task(&mut self) {
        if self.available_tasks.len() < 2 {
            return;
        }

        let running_task = self
            .available_tasks
            .pop_front()
            .unwrap();

        self.available_tasks
            .push_back(running_task);

        let next_task = self
            .available_tasks
            .front()
            .unwrap();

        let running_task = self
            .available_tasks
            .back()
            .unwrap();
        asm_switch_context(&next_task.context.0, &running_task.context.0);
    }


    fn remove_sleep_task_at(&mut self, task_id: u64) -> KernelResult<Task> {
        let task_index = self
            .sleep_tasks
            .iter()
            .position(|t| t.id == task_id)
            .ok_or(error_not_found_task(task_id))?;

        if self.sleep_tasks.len() <= task_index {
            Err(error_not_found_task(task_id))
        } else {
            Ok(self
                .sleep_tasks
                .remove(task_index)
                .ok_or(error_not_found_task(task_id))?)
        }
    }


    fn remove_available_task_at(&mut self, task_id: u64) -> KernelResult<Task> {
        let task_index = self
            .available_tasks
            .iter()
            .position(|t| t.id == task_id)
            .ok_or(error_not_found_task(task_id))?;

        if self.available_tasks.len() <= task_index {
            Err(error_not_found_task(task_id))
        } else {
            Ok(self
                .available_tasks
                .remove(task_index)
                .ok_or(error_not_found_task(task_id))?)
        }
    }


    fn front_available(&self) -> KernelResult<&Task> {
        self.available_tasks
            .front()
            .ok_or(kernel_error!("Available Tasks is Empty"))
    }


    #[inline]
    fn create_task(&self) -> Task {
        Task::new(self.available_tasks.len() as u64)
    }
}


#[inline]
fn error_not_found_task(task_id: u64) -> KernelError {
    kernel_error!("Not found Task: specified id = {task_id}")
}


#[derive(Debug)]
pub struct Task {
    id: u64,
    context: Context,
    stack: Box<[u8]>,
}


impl Task {
    pub fn new(id: u64) -> Self {
        Self {
            id,
            context: Context::uninit(),
            stack: vec![0; 65_536].into_boxed_slice(),
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


    pub fn cloned_context(&self) -> Context {
        self.context.clone()
    }
}

#[cfg(test)]
mod tests {}
