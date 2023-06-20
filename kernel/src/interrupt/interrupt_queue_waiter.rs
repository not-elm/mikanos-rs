use common_lib::queue::queueing::Queueing;
use kernel_lib::interrupt::asm::{cli, sti, sti_and_hlt};

use crate::interrupt::InterruptMessage;
use crate::interrupt::xhci::INTERRUPT_QUEUE;

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

        let mut value = unsafe { INTERRUPT_QUEUE.dequeue() };

        while value.is_none() {
            sti_and_hlt();

            cli();
            value = unsafe { INTERRUPT_QUEUE.dequeue() };
        }
        sti();
        value
    }
}
