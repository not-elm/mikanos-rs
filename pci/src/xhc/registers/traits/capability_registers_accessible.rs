pub trait CapabilityRegistersAccessible {
    fn read_max_scratchpad_buffers_len(&self) -> usize;
}
