use alloc::sync::Arc;

use spin::RwLock;

use crate::task::status::Status;
use crate::task::status::Status::Running;
use crate::task::Task;

#[derive(Debug, Clone)]
pub struct SwitchCommand {
    running: &Task,
    next: Task,
}


impl SwitchCommand {
    #[inline(always)]
    pub const fn new(
        running: Arc<RwLock<Task>>,
        next: Arc<RwLock<Task>>,
    ) -> SwitchCommand {
        Self {
            running,
            next,
        }
    }


    #[inline(always)]
    pub fn switch_and_sleep(&self) {
        self.switch(Status::Sleep);
    }


    #[inline(always)]
    pub fn switch_and_pending(&self) {
        self.switch(Status::Pending);
    }


    #[cfg(test)]
    pub(crate) fn running_id(&self) -> u64 {
        self.running.read().id
    }


    #[cfg(test)]
    pub(crate) fn next_id(&self) -> u64 {
        self.next.read().id
    }


    fn switch(&self, status: Status) {
        let mut running = self.running.write();
        let mut next = self.next.write();
        running.status = status;
        next.status = Running;

        self
            .running
            .read()
            .switch_to(&next)
    }
}