use kernel_lib::println;

use crate::error::PciResult;
use crate::xhc::allocator::memory_allocatable::MemoryAllocatable;
use crate::xhc::transfer::device_context::scratchpad_buffer_ptr::ScratchpadBufferPtr;

#[repr(transparent)]
pub struct ScratchpadBuffersArrayPtr(u64);

impl ScratchpadBuffersArrayPtr {
    pub fn new(
        scratchpad_buffers_len: usize,
        allocator: &mut impl MemoryAllocatable,
    ) -> PciResult<Self> {
        unsafe {
            Ok(Self(Self::allocate_scratchpad_buffers(
                scratchpad_buffers_len,
                allocator,
            )?))
        }
    }
    pub fn base_addr(&self) -> u64 {
        self.0
    }
    unsafe fn allocate_scratchpad_buffers(
        scratchpad_buffers_len: usize,
        allocator: &mut impl MemoryAllocatable,
    ) -> PciResult<u64> {
        let scratchpad_buffers_array_address =
            allocator.try_allocate_max_scratchpad_buffers(scratchpad_buffers_len)?;
        println!("scratchpad_buffers_len={}", scratchpad_buffers_len);
        for i in 0..scratchpad_buffers_len {
            let scratchpad_buff = scratchpad_buffers_array_address as *mut ScratchpadBufferPtr;
            let scratchpad_buff = scratchpad_buff.add(i);
            *(scratchpad_buff) =
                ScratchpadBufferPtr::new_with_allocate(scratchpad_buff as u64, allocator)?;
        }

        Ok(scratchpad_buffers_array_address)
    }
}
