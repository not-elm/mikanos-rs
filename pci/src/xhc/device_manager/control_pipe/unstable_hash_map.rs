use alloc::vec::Vec;

pub struct UnstableHashMap<Key, Value> {
    keys: Vec<Key>,
    values: Vec<Value>,
}

impl<Key, Value> UnstableHashMap<Key, Value>
where
    Key: Eq,
{
    pub fn new() -> Self {
        Self {
            keys: Vec::new(),
            values: Vec::new(),
        }
    }

    pub fn get(&self, key: Key) -> Option<&Value> {
        self.values.get(self.index(&key)?)
    }
    pub fn get_mut(&mut self, key: Key) -> Option<&mut Value> {
        let i = self.index(&key)?;
        self.values.get_mut(i)
    }
    pub fn set(&mut self, key: Key, value: Value) {
        if let Some(i) = self.index(&key) {
            self.keys[i] = key;
            self.values[i] = value;
        } else {
            self.keys.push(key);
            self.values.push(value);
        }
    }

    fn index(&self, key: &Key) -> Option<usize> {
        self.keys.iter().position(|k| *k == *key)
    }
}

#[cfg(test)]
mod tests {
    use crate::xhc::device_manager::control_pipe::unstable_hash_map::UnstableHashMap;
    use xhci::ring::trb::transfer::SetupStage;
    #[test]
    fn it_failed_when_empty() {
        let map: UnstableHashMap<i32, &str> = UnstableHashMap::new();

        assert!(map.get(3).is_none());
    }
    #[test]
    fn it_get() {
        let mut map: UnstableHashMap<i32, &str> = UnstableHashMap::new();

        map.set(3, "A");
        assert!(map.get(3).is_some_and(|v| *v == "A"));
    }

    #[test]
    fn it_override() {
        let mut map: UnstableHashMap<i32, &str> = UnstableHashMap::new();

        map.set(3, "A");
        map.set(3, "B");
        assert!(map.get(3).is_some_and(|v| *v == "B"));
    }

    #[test]
    fn it_mutable_get() {
        let mut map: UnstableHashMap<i32, SetupStage> = UnstableHashMap::new();

        map.set(3, SetupStage::new());
        let setup = map.get_mut(3).unwrap();
        setup.set_cycle_bit();

        assert!(map.get(3).is_some_and(|v| v.cycle_bit()));
    }
}
