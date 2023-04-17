use uefi::table::boot::{MemoryDescriptor, MemoryMapIter, MemoryType};

use common_lib::physical_address::PhysicalAddress;

use crate::allocator::FRAME_SIZE;
use crate::allocator::memory_map::frame_iterable::{MemoryMapFrame, MemoryMapFrameIterable};
use crate::allocator::memory_map::page_range::PageRange;

#[derive(Debug, Clone)]
pub struct FrameRange<'memory> {
    frame_id: usize,
    memory_map: MemoryMapIter<'memory>,
    page_range: PageRange,
    // FrameIDからアドレスを取得するために保持
    clone_memory_map: MemoryMapIter<'memory>,
}


impl<'memory> FrameRange<'memory> {
    pub fn new(mut memory_map: MemoryMapIter<'memory>) -> Option<FrameRange<'memory>> {
        let clone_memory_map = memory_map.clone();
        let first_descriptor = next_available(&mut memory_map)?;
        Some(FrameRange {
            frame_id: 0,
            memory_map,
            page_range: PageRange::new(first_descriptor),
            clone_memory_map,
        })
    }
}

impl<'memory> MemoryMapFrameIterable for FrameRange<'memory> {
    fn last_id(&self) -> Option<usize> {
        let clone = Self::new(self.clone_memory_map.clone())?;
        let frame = clone.into_iter().last()?;
        Some(frame.id())
    }

    fn frame_at(&mut self, frame_id: usize) -> Option<MemoryMapFrame> {
        let clone = Self::new(self.clone_memory_map.clone())?;
        let frame = clone.into_iter().find(|frame| frame.id() == frame_id)?;
        Some(frame)
    }

    fn frame_contains_address(&mut self, phys_addr: PhysicalAddress) -> Option<MemoryMapFrame> {
        let clone = Self::new(self.clone_memory_map.clone())?;
        let frame = clone.into_iter().find(|frame| frame.contains(phys_addr))?;
        Some(frame)
    }
}

impl<'memory> Iterator for FrameRange<'memory> {
    type Item = MemoryMapFrame;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(phys_addr) = self.page_range.next() {
            let frame = MemoryMapFrame::new(self.frame_id, phys_addr, phys_addr.add_u64(FRAME_SIZE as u64)?);
            self.frame_id += 1;
            Some(frame)
        } else {
            self.page_range = PageRange::new(next_available(&mut self.memory_map)?);
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
    };
}

fn is_available(memory_descriptor: &MemoryDescriptor) -> bool {
    matches!(
        memory_descriptor.ty,
        MemoryType::CONVENTIONAL
    )
}
