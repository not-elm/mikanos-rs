use crate::task::status::Status;
use crate::task::status::Status::Running;
use crate::task::Task;

#[derive(Debug)]
pub struct SwitchCommand<'t> {
    running: &'t Task,
    next: &'t Task,
}


impl<'t> SwitchCommand<'t> {
    #[inline(always)]
    pub fn new(running: &'t Task, next: &'t Task) -> SwitchCommand<'t> {
        Self { running, next }
    }


    #[inline(always)]
    pub fn switch_and_sleep(&mut self) {
        self.switch(Status::Sleep);
    }


    #[inline(always)]
    pub fn switch_and_pending(&mut self) {
        self.switch(Status::Pending);
    }


    #[cfg(test)]
    pub(crate) fn running_id(&self) -> u64 {
        self.running.id
    }


    #[cfg(test)]
    pub(crate) fn next_id(&self) -> u64 {
        self.next.id
    }


    pub fn switch_if_need(&self, status: Status) {
        if self.next.priority_level < self.running.priority_level {
            return;
        }

        self.switch(status);
    }


    fn switch(&self, status: Status) {
        self.running
            .store_status(status);
        self.next
            .store_status(Running);

        self.running
            .switch_to(self.next)
    }
}
