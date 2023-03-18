use crate::xhci::memory_allocatable::MemoryAllocatable;

const MEMORY_SIZE: usize = 4096 * 32;
static mut MEMORY_POOL: MemoryPool = MemoryPool([0; MEMORY_SIZE]);

#[repr(align(64))]
pub struct MemoryPool([u8; MEMORY_SIZE]);

#[derive(Debug)]
pub struct MikanOSPciMemoryAllocator {
    index: usize,
}

impl MemoryAllocatable for MikanOSPciMemoryAllocator {
    unsafe fn alloc(&mut self, bytes: usize) -> Option<usize> {
        if MEMORY_POOL.0.len() <= self.index {
            return None;
        }
        let memory_buff = MEMORY_POOL.0;
        let base = (memory_buff[self.index] as *mut u8).addr();

        self.index += add_index_with_align(self.index, bytes);

        Some(base)
    }

    unsafe fn free(&mut self, _base_addr: usize) {}
}

fn add_index_with_align(index: usize, bytes: usize) -> usize {
    let diff = bytes % 64;
    if diff == 0 {
        index + bytes
    } else {
        index + bytes + (64 - diff)
    }
}
