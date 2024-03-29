use alloc::vec;
use alloc::vec::Vec;

use crate::iter::group::Group;

pub struct MutableGroup<Key, Value> {
    key: Key,
    values: Vec<Value>,
}

impl<Key, Value> MutableGroup<Key, Value> {
    pub fn empty(key: Key) -> MutableGroup<Key, Value> {
        Self {
            key,
            values: Vec::new(),
        }
    }


    pub fn new(key: Key, value: Value) -> MutableGroup<Key, Value> {
        Self {
            key,
            values: vec![value],
        }
    }


    pub fn key_ref(&self) -> &Key {
        &self.key
    }


    pub fn push_value(&mut self, value: Value) {
        self.values.push(value);
    }


    pub fn into_immutable_group(self) -> Group<Key, Value> {
        Group::new(self.key, self.values)
    }
}


impl<Key, Value> MutableGroup<Key, Value>
where
    Key: PartialEq,
{
    pub fn eq_key(&self, key: &Key) -> bool {
        self.key == *key
    }
}


#[cfg(test)]
mod tests {
    use crate::iter::mutable_group::MutableGroup;

    #[test]
    fn it_eq_key() {
        let key = "hello";
        let group = MutableGroup::new(key, 1);
        assert!(group.eq_key(&key))
    }
}
