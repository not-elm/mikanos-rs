use crate::interrupt::asm::{cli, sti};
use crate::serial_println;
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
    pub fn new(
        running: &'t Task,
        next: &'t Task,
    ) -> SwitchCommand<'t> {
        Self {
            running,
            next,
        }
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


    fn switch(&self, status: Status) {
        cli();
        self.running.status.set(status);
        self.next.status.set(Running);
        sti();
        
        self
            .running
            .switch_to(self.next)
    }
}