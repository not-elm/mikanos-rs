use crate::error::OldPciResult;

pub trait DoorbellRegistersAccessible {
    fn notify_at(&mut self, index: usize, target: u8, stream_id: u16) -> OldPciResult;
}
