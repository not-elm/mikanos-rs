use kernel_lib::interrupt::asm::{cli, sti, sti_and_hlt, sti_and_nop};
use kernel_lib::interrupt::interrupt_message::TaskMessage;

use crate::task::TASK_MANAGER;

pub struct TaskMessageIter {
    task_id: u64,
}

impl TaskMessageIter {
    #[inline(always)]
    pub const fn new(task_id: u64) -> Self {
        Self { task_id }
    }
}


impl Iterator for TaskMessageIter {
    type Item = TaskMessage;

    fn next(&mut self) -> Option<Self::Item> {
        cli();

        let mut value = unsafe { TASK_MANAGER.receive_message_at(self.task_id) };

        while value.is_none() {
            // unsafe {
            //     TASK_MANAGER
            //         .sleep_at(self.task_id)
            //         .unwrap()
            // };
            sti_and_hlt();

            cli();
            value = unsafe { TASK_MANAGER.receive_message_at(self.task_id) };
        }

        sti_and_nop();
        value
    }
}