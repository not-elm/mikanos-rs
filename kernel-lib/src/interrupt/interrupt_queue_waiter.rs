use core::marker::PhantomData;

use common_lib::queue::queueing::Queueing;

use crate::interrupt::{cli, sti_and_hlt};

pub struct InterruptQueueWaiter<Queue, Value>
where
    Queue: Queueing<Value> + 'static,
{
    queue: &'static mut Queue,
    _maker: PhantomData<Value>,
}

impl<Queue, Value> InterruptQueueWaiter<Queue, Value>
where
    Queue: Queueing<Value> + 'static,
{
    pub fn new(queue: &'static mut Queue) -> InterruptQueueWaiter<Queue, Value> {
        Self {
            queue,
            _maker: PhantomData,
        }
    }
}

impl<Queue, Value> Iterator for InterruptQueueWaiter<Queue, Value>
where
    Queue: Queueing<Value>,
{
    type Item = Value;

    fn next(&mut self) -> Option<Self::Item> {
        let mut value = self.queue.dequeue();
        while value.is_none() {
            sti_and_hlt();

            value = self.queue.dequeue();
        }
        cli();
        Some(value.unwrap())
    }
}
