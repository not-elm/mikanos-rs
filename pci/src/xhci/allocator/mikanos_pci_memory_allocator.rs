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
    unsafe fn allocate_with_align(
        &mut self,
        bytes: usize,
        align: usize,
        page_bounds: usize,
    ) -> Option<AlignedAddress> {
        if MEMORY_POOL.0.len() <= self.index {
            return None;
        }
        let memory_buff = MEMORY_POOL.0;

        let allocated_memory_base_addr = (memory_buff[self.index] as *mut u8).addr();

        if 0 < align {
            self.index = add_index_with_align(self.index, bytes, align);
        }
        if 0 < page_bounds {
            self.index = next_bound_if_over_allocate_current(self.index, bytes, page_bounds);
        }

        Some(AlignedAddress::new_with_check_align_64_bytes(allocated_memory_base_addr).ok()?)
    }

    unsafe fn free(&mut self, _base_addr: usize) {}
}

fn add_index_with_align(index: usize, bytes: usize, align: usize) -> usize {
    let diff = bytes % align;
    if diff == 0 {
        index + bytes
    } else {
        index + bytes + (align - diff)
    }
}

fn next_bound_if_over_allocate_current(index: usize, bytes: usize, bound: usize) -> usize {
    let diff = index % bound;
    if (diff + bytes) <= bound {
        index
    } else {
        index + (bound - diff)
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
        let addr = unsafe { allocator.allocate_with_align(32, 64, 64 * 1024) };
        assert!(addr
            .map(|ptr_addr| ptr_addr.address().unwrap() == base_addr)
            .is_some());
        assert_eq!(allocator.index, 64);
    }

    #[test]
    fn it_align_more_than_64bytes() {
        let mut allocator = MikanOSPciMemoryAllocator::new();
        let base_addr = unsafe { MEMORY_POOL.0.as_ptr().addr() };
        let addr = unsafe { allocator.allocate_with_align(65, 64, 64 * 1024) };
        assert!(addr
            .map(|ptr_addr| ptr_addr.address().unwrap() == base_addr)
            .is_some());
        assert_eq!(allocator.index, 128);
    }

    #[test]
    fn it_over_bound() {
        let mut allocator = MikanOSPciMemoryAllocator::new();
        let base_addr = unsafe { MEMORY_POOL.0.as_ptr().addr() };
        let addr = unsafe { allocator.allocate_with_align(13, 10, 15) };
        assert!(addr
            .map(|ptr_addr| ptr_addr.address().unwrap() == base_addr)
            .is_some());
        assert_eq!(allocator.index, 30);
    }
}
