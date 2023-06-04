use core::cmp::Ordering;

use derive_getters::Getters;

#[derive(Debug, Clone, Getters)]
pub struct Timeout<T> {
    timeout: usize,

    /// タイムアウト時に送信される値
    value: T,
}


impl<T> Timeout<T> {
    pub const fn new(timeout: usize, value: T) -> Timeout<T> {
        Self { timeout, value }
    }
}


impl<Value> PartialEq<Self> for Timeout<Value>
where
    Value: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.timeout == other.timeout && self.value == other.value
    }
}


impl<Value> Eq for Timeout<Value> where Value: PartialEq {}


impl<Value> PartialOrd for Timeout<Value>
where
    Value: PartialEq,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self
            .timeout
            .partial_cmp(&other.timeout)?
        {
            Ordering::Less => Some(Ordering::Greater),
            Ordering::Equal => Some(Ordering::Equal),
            Ordering::Greater => Some(Ordering::Less),
        }
    }
}


impl<Value> Ord for Timeout<Value>
where
    Value: PartialEq,
{
    fn cmp(&self, other: &Self) -> Ordering {
        match {
            self.timeout
                .cmp(&other.timeout)
        } {
            Ordering::Less => Ordering::Greater,
            Ordering::Equal => Ordering::Equal,
            Ordering::Greater => Ordering::Less,
        }
    }
}
