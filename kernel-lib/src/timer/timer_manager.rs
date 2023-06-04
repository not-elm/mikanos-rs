use alloc::collections::BinaryHeap;
use alloc::vec::Vec;

use crate::timer::apic::timeout::Timeout;

#[derive(Default)]
pub struct TimeOutManager<Value>
where
    Value: Ord,
{
    timeouts: BinaryHeap<Timeout<Value>>,
    tick: usize,
}


impl<Value> TimeOutManager<Value>
where
    Value: Ord + Clone,
{
    pub fn push_timeout(&mut self, timeout: Timeout<Value>) {
        self.timeouts.push(timeout);
    }


    pub fn tick(&mut self) -> Option<Vec<Value>> {
        let mut timeouts = Vec::<Value>::with_capacity(self.timeouts.len());
        self.tick += 1;

        while let Some(t) = self.timeouts.peek() {
            if self.tick < *t.timeout() {
                break;
            }
            timeouts.push(
                self.timeouts
                    .pop()?
                    .value()
                    .clone(),
            );
        }

        none_if_empty(timeouts)
    }
}


fn none_if_empty<Value>(vec: Vec<Value>) -> Option<Vec<Value>> {
    if vec.is_empty() {
        None
    } else {
        Some(vec)
    }
}


#[cfg(test)]
mod tests {
    use crate::timer::apic::timeout::Timeout;
    use crate::timer::timer_manager::TimeOutManager;

    #[test]
    fn it_timeout_1() {
        let mut manager = TimeOutManager::<u32>::default();
        manager.push_timeout(Timeout::new(3, 3));
        manager.push_timeout(Timeout::new(10, 10));

        assert!(manager.tick().is_none());
        assert!(manager.tick().is_none());

        let timeouts = manager.tick().unwrap();
        assert_eq!(timeouts.len(), 1);
        assert_eq!(timeouts[0], 3)
    }


    #[test]
    fn it_timeouts_2() {
        let mut manager = TimeOutManager::<u32>::default();
        manager.push_timeout(Timeout::new(3, 3));
        manager.push_timeout(Timeout::new(3, 10));

        assert!(manager.tick().is_none());
        assert!(manager.tick().is_none());

        let timeouts = manager.tick().unwrap();
        assert_eq!(timeouts.len(), 2);
        assert_eq!(timeouts[0], 3);
        assert_eq!(timeouts[1], 10);
    }
}
