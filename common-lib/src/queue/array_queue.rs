use crate::queue::queueing::Queueing;


#[derive(Debug)]
pub struct ArrayQueue<T: Copy> {
    queue: [Option<T>; 1024],
    last_index: usize,
    front_index: usize,
}

impl<Value: Copy> Queueing<Value> for ArrayQueue<Value> {
    fn enqueue(&mut self, value: Value) {
        if self.queue.len() <= self.last_index {
            self.last_index = 0;
        }
        self.queue[self.last_index] = Some(value);
        self.last_index += 1;
    }

    fn dequeue(&mut self) -> Option<Value> {
        if self.queue.len() <= self.front_index {
            self.front_index = 0;
        }

        let v = self.queue[self.front_index];
        self.front_index += 1;
        v
    }
}

impl<T: Copy> ArrayQueue<T> {
    pub const fn new() -> Self {
        Self {
            queue: [None; 1024],
            last_index: 0,
            front_index: 0,
        }
    }
}
