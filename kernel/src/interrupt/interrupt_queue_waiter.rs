use kernel_lib::interrupt::asm::{cli, sti, sti_and_hlt};
use kernel_lib::interrupt::interrupt_message::TaskMessage;

use crate::task::TASK_MANAGER;

pub struct InterruptQueueWaiter;

impl InterruptQueueWaiter {
    pub fn new() -> Self {
        Self {}
    }
}

impl Iterator for InterruptQueueWaiter {
    type Item = TaskMessage;

    fn next(&mut self) -> Option<Self::Item> {
        cli();

        let mut value = unsafe { TASK_MANAGER.receive_message_at(0) };

        while value.is_none() {
            sti_and_hlt();

            cli();
            value = unsafe { TASK_MANAGER.receive_message_at(0) };
        }
        sti();
        value
    }
}
