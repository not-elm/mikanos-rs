use alloc::vec::Vec;
use core::fmt::{Debug, Formatter};

pub struct Group<Key, Value> {
    key: Key,
    values: Vec<Value>,
}


impl<Key, Value> Group<Key, Value> {
    pub fn new(key: Key, values: Vec<Value>) -> Group<Key, Value> {
        Self { key, values }
    }


    pub fn values_ref(&self) -> &Vec<Value> {
        &self.values
    }


    pub fn into_values(self) -> Vec<Value> {
        self.values
    }
}


impl<Key, Value> Group<Key, Value>
where
    Key: PartialEq,
{
    pub fn eq_key(&self, key: &Key) -> bool {
        self.key == *key
    }
}


impl<Key, Value> Debug for Group<Key, Value>
where
    Key: Debug,
    Value: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Group")
            .field("key", &self.key)
            .field("values", &self.values)
            .finish()
    }
}


impl<Key, Value> Clone for Group<Key, Value>
where
    Key: Clone,
    Value: Clone,
{
    fn clone(&self) -> Self {
        Group::new(self.key.clone(), self.values.clone())
    }
}
