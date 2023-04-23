pub trait KeyQuery<Key, Value>
where
    Key: PartialEq,
{
    fn query(&mut self, value: Value) -> Key;
}


impl<F, Key, Value> KeyQuery<Key, Value> for F
where
    F: FnMut(Value) -> Key,
    Key: PartialEq,
{
    fn query(&mut self, value: Value) -> Key {
        self(value)
    }
}
