use uefi::table::boot::MemoryDescriptor;

use crate::allocator::FRAME_SIZE;

#[derive(Debug, Clone)]
pub struct MemoryMapFrame {
    frame_id: usize,
    descriptor: MemoryDescriptor,
}


impl MemoryMapFrame {
    pub fn new(frame_id: usize, descriptor: MemoryDescriptor) -> MemoryMapFrame {
        Self {
            frame_id,
            descriptor,
        }
    }
    pub fn id(&self) -> usize {
        self.frame_id
    }

    pub fn descriptor(&self) -> MemoryDescriptor {
        self.descriptor
    }
}

pub trait MemoryMapFrameIterable: Iterator<Item = MemoryMapFrame> {
    fn last_id(&self) -> usize;
    fn get(&mut self, frame_id: usize) -> Option<MemoryMapFrame>;
}


pub fn get_contains_address<MemoryMap>(
    memory_map: &mut MemoryMap,
    phs_addr: u64,
) -> Option<MemoryMapFrame>
where
    MemoryMap: MemoryMapFrameIterable,
{
    memory_map.find(|frame| {
        let frame_addr = frame.descriptor.phys_start;
        frame_addr <= phs_addr && phs_addr <= (frame_addr + FRAME_SIZE as u64)
    })
}
