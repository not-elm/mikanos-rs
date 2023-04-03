use alloc::vec::Vec;

pub struct UnstableHashMap<Key, Value> {
    keys: Vec<Key>,
    values: Vec<Value>,
}

impl<Key, Value> UnstableHashMap<Key, Value> {
    pub fn new() -> Self {
        Self {
            keys: Vec::new(),
            values: Vec::new(),
        }
    }

    pub fn get(&self, key: Key) -> Option<Value> {
        let i = self.keys.iter().position(key)?;
        self.values.get(i)
    }

    pub fn set(&mut self, key: Key, value: Value) {
        if let Some(i) = self.keys.iter().position(&key) {
            self.keys[i] = key;
            self.values[i] = value;
        } else {
            self.keys.push(key);
            self.values.push(value);
        }
    }
}
