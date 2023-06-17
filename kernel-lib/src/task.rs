use alloc::vec;
use alloc::vec::Vec;
use core::cell::OnceCell;

use crate::context::arch::x86_64::Context;

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


    #[inline(always)]
    pub fn switch_task(&mut self) {
        if let Some(manager) = self.0.get_mut() {
            manager.switch_task();
        }
    }


    pub fn tasks(&self) -> &[Task] {
        &self.0.get().unwrap().tasks
    }
}


unsafe impl Sync for CellTaskManger {}


#[derive(Default, Debug)]
pub struct TaskManager {
    tasks: Vec<Task>,
    running_idx: usize,
}


impl TaskManager {
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            tasks: vec![Task::new(0)],
            running_idx: 0,
        }
    }


    pub fn new_task(&mut self) -> &mut Task {
        self.tasks
            .push(self.create_task());

        self.tasks.last_mut().unwrap()
    }


    #[inline]
    fn create_task(&self) -> Task {
        Task::new(self.tasks.len() as u64)
    }


    #[inline(always)]
    pub fn switch_task(&mut self) {
        if self.tasks.len() < 2 {
            return;
        }
        let current_idx = self.running_idx;
        let next_idx = (current_idx + 1) % self.tasks.len();
        self.running_idx = next_idx;
        let current_ctx = self
            .tasks
            .get(current_idx)
            .unwrap();

        let next_ctx = self
            .tasks
            .get(next_idx)
            .unwrap();

        current_ctx.switch_to(next_ctx);
    }


    pub fn tasks(&mut self) -> &mut Vec<Task> {
        &mut self.tasks
    }
}


#[derive(Debug)]
pub struct Task {
    id: u64,
    context: Context,
    stack: Vec<u64>,
}


impl Task {
    pub fn new(id: u64) -> Self {
        Self {
            id,
            context: Context::uninit(),
            stack: vec![0; 4096 / 8],
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
}

#[cfg(test)]
mod tests {}
