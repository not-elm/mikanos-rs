use crate::xhc::allocator::aligned_address::AlignedAddress;
use crate::xhc::allocator::memory_allocatable::MemoryAllocatable;

const MEMORY_SIZE: usize = 4096 * 32;
static mut MEMORY_POOL: MemoryPool = MemoryPool([0u8; MEMORY_SIZE]);

#[repr(C, align(64))]
#[derive(Debug)]
pub struct MemoryPool([u8; MEMORY_SIZE]);

#[derive(Debug)]
pub struct MikanOSPciMemoryAllocator {
    address: usize,
    end_address: usize,
}

impl MikanOSPciMemoryAllocator {
    pub fn new() -> Self {
        let address = unsafe { MEMORY_POOL.0.as_ptr().addr() };

        Self {
            address,
            end_address: address + MEMORY_SIZE,
        }
    }

    unsafe fn align_ptr(&self, align: usize) -> *const u8 {
        let ptr = self.address as *const u8;
        if align > 0 && !ptr.is_aligned_to(align) {
            ptr.add(ptr.align_offset(align))
        } else {
            ptr
        }
    }
    fn end_addr(&self) -> usize {
        // let buff = unsafe { self.memory_pool.0 };
        //
        // core::ptr::addr_of!(buff[buff.len() - 1]).addr()
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
        if self.end_addr() < self.address + bytes {
            return None;
        }
        let align_ptr = self.align_ptr(align);
        let align_ptr = step_next_bound_if_over(align_ptr, bytes, page_bounds);

        let next_ptr = align_ptr.byte_add(bytes);

        if self.end_addr() < next_ptr.addr() {
            return None;
        }

        let allocated_memory_base_addr: usize = align_ptr as usize;

        self.address = next_ptr as usize;
        Some(AlignedAddress::new_uncheck(allocated_memory_base_addr))
    }

    unsafe fn free(&mut self, _base_addr: usize) {}
}

unsafe fn step_next_bound_if_over(ptr: *const u8, bytes: usize, bound: usize) -> *const u8 {
    if bound == 0 {
        return ptr;
    }

    let diff = bound % ptr.addr();
    let next_bound = bound - diff;
    if next_bound < bytes {
        ptr.byte_add(next_bound)
    } else {
        ptr
    }
}

#[cfg(test)]
mod tests {}
