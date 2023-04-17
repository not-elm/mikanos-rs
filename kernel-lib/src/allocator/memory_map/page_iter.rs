use uefi::table::boot::MemoryDescriptor;
use common_lib::physical_address::PhysicalAddress;

use crate::allocator::FRAME_SIZE;

#[derive(Debug, Clone)]
pub struct PageIter {
    frame: MemoryDescriptor,
    page_no: u64,
}


impl PageIter {
    pub fn new(frame: MemoryDescriptor) -> Self {
        Self {
            frame,
            page_no: 0,
        }
    }
}


impl Iterator for PageIter {
    type Item = PhysicalAddress;

    fn next(&mut self) -> Option<Self::Item> {
        if self.frame.page_count <= self.page_no {
            return None;
        }

        let phys_addr = self.frame.phys_start + (self.page_no * FRAME_SIZE as u64);
        self.page_no += 1;
        Some(PhysicalAddress::new(phys_addr))
    }
}
