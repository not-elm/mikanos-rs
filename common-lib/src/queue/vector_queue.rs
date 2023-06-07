use alloc::collections::VecDeque;

use crate::queue::queueing::Queueing;

#[derive(Debug)]
pub struct VectorQueue<T> {
    queue: VecDeque<T>,
}

impl<Value> Queueing<Value> for VectorQueue<Value> {
    fn enqueue(&mut self, value: Value) {
        self.queue.push_front(value);
    }

    fn dequeue(&mut self) -> Option<Value> {
        self.queue.pop_front()
    }
}


impl<T> VectorQueue<T> {
    pub const fn new() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }


    pub fn count(&self) -> usize {
        self.queue.len()
    }
}
