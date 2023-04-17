use crate::allocator::memory_map::frame::Frame;
use common_lib::physical_address::PhysicalAddress;

pub trait MemoryMapFrameIterable: Iterator<Item = Frame> {
    fn last_id(&self) -> Option<usize>;
    fn frame_at(&mut self, frame_id: usize) -> Option<Frame>;
    fn frame_contains_address(&mut self, phys_addr: PhysicalAddress) -> Option<Frame>;
}
