use crate::xhci::allocator::aligned_address::AlignedAddress;
use crate::xhci::allocator::memory_allocatable::MemoryAllocatable;

const MEMORY_SIZE: usize = 4096 * 32;
static mut MEMORY_POOL: MemoryPool = MemoryPool([0; MEMORY_SIZE]);

#[repr(align(64))]
pub struct MemoryPool([u8; MEMORY_SIZE]);

#[derive(Debug)]
pub struct MikanOSPciMemoryAllocator {
    index: usize,
}

impl MikanOSPciMemoryAllocator {
    pub fn new() -> Self {
        Self { index: 0 }
    }
}

impl MemoryAllocatable for MikanOSPciMemoryAllocator {
    unsafe fn allocate_with_align_64_bytes(&mut self, bytes: usize) -> Option<AlignedAddress> {
        if MEMORY_POOL.0.len() <= self.index {
            return None;
        }
        let memory_buff = MEMORY_POOL.0;
        let allocated_memory_base_addr = (memory_buff[self.index] as *mut u8).addr();

        self.index = add_index_with_align(self.index, bytes);

        Some(AlignedAddress::new_with_check_align_64_bytes(allocated_memory_base_addr).ok()?)
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

#[cfg(test)]
mod tests {
    use crate::xhci::allocator::memory_allocatable::MemoryAllocatable;
    use crate::xhci::allocator::mikanos_pci_memory_allocator::{
        MikanOSPciMemoryAllocator, MEMORY_POOL,
    };

    #[test]
    fn it_align() {
        let mut allocator = MikanOSPciMemoryAllocator::new();
        let base_addr = unsafe { MEMORY_POOL.0.as_ptr().addr() };
        let addr = unsafe { allocator.allocate_with_align_64_bytes(32) };
        assert!(addr
            .map(|ptr_addr| ptr_addr.address().unwrap() == base_addr)
            .is_some());
        assert_eq!(allocator.index, 64);
    }

    #[test]
    fn it_align_more_than_64bytes() {
        let mut allocator = MikanOSPciMemoryAllocator::new();
        let base_addr = unsafe { MEMORY_POOL.0.as_ptr().addr() };
        let addr = unsafe { allocator.allocate_with_align_64_bytes(65) };
        assert!(addr
            .map(|ptr_addr| ptr_addr.address().unwrap() == base_addr)
            .is_some());
        assert_eq!(allocator.index, 128);
    }
}
