use uefi::table::boot::MemoryDescriptor;

use crate::allocator::FRAME_SIZE;

#[derive(Debug, Clone)]
pub struct MemoryMapFrame<'memory> {
    frame_id: usize,
    descriptor: &'memory MemoryDescriptor,
}


impl<'memory> MemoryMapFrame<'memory> {
    pub fn new(frame_id: usize, descriptor: &'memory MemoryDescriptor) -> MemoryMapFrame<'memory> {
        Self {
            frame_id,
            descriptor,
        }
    }
    pub fn id(&self) -> usize {
        self.frame_id
    }

    pub fn descriptor(&self) -> &'memory MemoryDescriptor {
        self.descriptor
    }
}

pub trait MemoryMapFrameIterable<'memory>: Iterator<Item = MemoryMapFrame<'memory>> {
    fn last_id(&self) -> usize;
    fn get(&mut self, frame_id: usize) -> Option<MemoryMapFrame<'memory>>;
}


pub fn get_contains_address<'memory, MemoryMap>(
    memory_map: &'memory mut MemoryMap,
    phs_addr: u64,
) -> Option<MemoryMapFrame<'memory>>
where
    MemoryMap: MemoryMapFrameIterable<'memory>,
{
    memory_map.find(|frame| {
        let frame_addr = frame.descriptor.phys_start;
        frame_addr <= phs_addr && phs_addr <= (frame_addr + FRAME_SIZE as u64)
    })
}
