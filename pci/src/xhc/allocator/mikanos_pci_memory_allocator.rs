use crate::xhc::allocator::aligned_address::AlignedAddress;
use crate::xhc::allocator::memory_allocatable::MemoryAllocatable;

const MEMORY_SIZE: usize = 4096 * 32;
static PCI_MEMORY_POOL: MemoryPool = MemoryPool([0u8; MEMORY_SIZE]);

#[repr(C, align(64))]
#[derive(Debug)]
pub struct MemoryPool([u8; MEMORY_SIZE]);

#[derive(Debug)]
pub struct MikanOSPciMemoryAllocator {
    address: u64,
    end_address: u64,
}

impl MikanOSPciMemoryAllocator {
    pub fn new() -> Self {
        let address = PCI_MEMORY_POOL.0.as_ptr() as u64;

        Self {
            address,
            end_address: address + MEMORY_SIZE as u64,
        }
    }

    unsafe fn align_ptr(&self, align: usize) -> *mut u8 {
        let ptr = self.address as *mut u8;
        if align > 0 && !ptr.is_aligned_to(align) {
            ptr.add(ptr.align_offset(align))
        } else {
            ptr
        }
    }
    fn end_addr(&self) -> u64 {
        self.end_address
    }
}

impl Default for MikanOSPciMemoryAllocator {
    fn default() -> Self {
        Self::new()
    }
}

impl MemoryAllocatable for MikanOSPciMemoryAllocator {
    unsafe fn allocate_with_align(
        &mut self,
        bytes: usize,
        align: usize,
        page_bounds: usize,
    ) -> Option<AlignedAddress> {
        if self.end_addr() < self.address + bytes as u64 {
            return None;
        }
        let align_ptr = self.align_ptr(align);
        let align_ptr = step_next_bound_if_over(align_ptr, bytes, page_bounds);

        let next_ptr = align_ptr
            .byte_add(bytes)
            .add(0x1000);

        if self.end_addr() < next_ptr as u64 {
            return None;
        }

        let allocated_memory_base_addr = align_ptr as u64;
        unsafe {
            let buff = core::slice::from_raw_parts_mut(align_ptr, bytes);
            buff.fill(0);
        }

        self.address = next_ptr as u64;
        Some(AlignedAddress::new_uncheck(allocated_memory_base_addr))
    }

    unsafe fn free(&mut self, _base_addr: u64, _bytes: usize) {
        todo!("not impl MemoryAllocatable::free")
    }
}

unsafe fn step_next_bound_if_over(ptr: *mut u8, bytes: usize, bound: usize) -> *mut u8 {
    if bound == 0 {
        return ptr;
    }

    let diff = ptr.addr() % bound;
    if diff == 0 {
        return ptr;
    }
    let next_bound = bound - diff;
    if next_bound < bytes {
        ptr.byte_add(next_bound)
    } else {
        ptr
    }
}
