use uefi::table::boot::{MemoryDescriptor, MemoryMapIter, MemoryType};

use crate::allocator::memory_map_frame_iterable::{MemoryMapFrame, MemoryMapFrameIterable};

#[derive(Debug, Clone)]
pub struct MemoryMapRange<'memory> {
    iter: MemoryMapIter<'memory>,
    frame_id: usize,
}


impl<'memory> MemoryMapRange<'memory> {
    pub fn new(iter: MemoryMapIter<'memory>) -> MemoryMapRange<'memory> {
        MemoryMapRange { iter, frame_id: 0 }
    }
}


impl<'memory> MemoryMapFrameIterable for MemoryMapRange<'memory> {
    fn last_id(&self) -> usize {
        self.iter.len() - 1
    }

    fn get(&mut self, frame_id: usize) -> Option<MemoryMapFrame> {
        self.nth(frame_id)
    }
}

impl<'memory> Iterator for MemoryMapRange<'memory> {
    type Item = MemoryMapFrame;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let descriptor = self.iter.next()?;
            self.frame_id += 1;
            if is_available(descriptor) {
                return Some(MemoryMapFrame::new(self.frame_id - 1, *descriptor));
            }
        }
    }
}


fn is_available(memory_descriptor: &MemoryDescriptor) -> bool {
    matches!(
        memory_descriptor.ty,
        MemoryType::CONVENTIONAL | MemoryType::BOOT_SERVICES_CODE | MemoryType::BOOT_SERVICES_DATA
    )
}
