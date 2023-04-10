pub trait Queueing<V> {
    /// Queueに値をエントリーします。
    fn enqueue(&mut self, value: V);


    /// Queueから値をデキューします。
    fn dequeue(&mut self) -> Option<V>;
}
