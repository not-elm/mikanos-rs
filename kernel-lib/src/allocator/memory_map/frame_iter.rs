use uefi::table::boot::{MemoryDescriptor, MemoryMapIter, MemoryType};

use common_lib::physical_address::PhysicalAddress;

use crate::allocator::FRAME_SIZE;
use crate::allocator::memory_map::frame::Frame;
use crate::allocator::memory_map::frame_iterable::MemoryMapFrameIterable;
use crate::allocator::memory_map::page_iter::PageIter;

#[derive(Debug, Clone)]
pub struct FrameIter<'memory> {
    frame_id: usize,
    memory_map: MemoryMapIter<'memory>,
    page_range: PageIter,
    // FrameIDからアドレスを取得するために保持
    clone_memory_map: MemoryMapIter<'memory>,
}


impl<'memory> FrameIter<'memory> {
    pub fn new(mut memory_map: MemoryMapIter<'memory>) -> Option<FrameIter<'memory>> {
        let clone_memory_map = memory_map.clone();
        // KERNEL自身の領域
        next_available(&mut memory_map)?;
        next_available(&mut memory_map)?;
        let first_descriptor = next_available(&mut memory_map)?;
        Some(FrameIter {
            frame_id: 0,
            memory_map,
            page_range: PageIter::new(first_descriptor),
            clone_memory_map,
        })
    }
}

impl<'memory> MemoryMapFrameIterable for FrameIter<'memory> {
    fn last_id(&self) -> Option<usize> {
        let clone = Self::new(self.clone_memory_map.clone())?;
        let frame = clone.into_iter().last()?;
        Some(frame.id())
    }

    fn frame_at(&mut self, frame_id: usize) -> Option<Frame> {
        let clone = Self::new(self.clone_memory_map.clone())?;
        let frame = clone
            .into_iter()
            .find(|frame| frame.id() == frame_id)?;
        Some(frame)
    }

    fn frame_contains_address(&mut self, phys_addr: PhysicalAddress) -> Option<Frame> {
        let clone = Self::new(self.clone_memory_map.clone())?;
        let frame = clone
            .into_iter()
            .find(|frame| frame.contains(phys_addr))?;
        Some(frame)
    }
}

impl<'memory> Iterator for FrameIter<'memory> {
    type Item = Frame;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(phys_addr) = self.page_range.next() {
            let frame = Frame::new(
                self.frame_id,
                phys_addr,
                phys_addr.add_u64(FRAME_SIZE as u64)?,
            );
            self.frame_id += 1;
            Some(frame)
        } else {
            self.page_range = PageIter::new(next_available(&mut self.memory_map)?);
            self.next()
        }
    }
}


fn next_available(memory_map: &mut MemoryMapIter) -> Option<MemoryDescriptor> {
    loop {
        let frame = memory_map.next()?;
        if is_available(frame) {
            return Some(*frame);
        }
    }
}

fn is_available(memory_descriptor: &MemoryDescriptor) -> bool {
    matches!(
        memory_descriptor.ty,
        MemoryType::CONVENTIONAL
    )
}
