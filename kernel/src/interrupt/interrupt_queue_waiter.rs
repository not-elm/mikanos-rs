use common_lib::queue::queueing::Queueing;
use kernel_lib::interrupt::asm::{cli, sti, sti_and_hlt};

use crate::interrupt::mouse::INTERRUPT_QUEUE;
use crate::interrupt::InterruptMessage;

pub struct InterruptQueueWaiter;

impl InterruptQueueWaiter {
    pub fn new() -> Self {
        Self {}
    }
}

impl Iterator for InterruptQueueWaiter {
    type Item = InterruptMessage;

    fn next(&mut self) -> Option<Self::Item> {
        cli();

        let mut value = INTERRUPT_QUEUE
            .lock()
            .borrow_mut()
            .dequeue();

        while value.is_none() {
            sti_and_hlt();

            cli();
            value = INTERRUPT_QUEUE
                .lock()
                .borrow_mut()
                .dequeue();
        }


        value
    }
}
