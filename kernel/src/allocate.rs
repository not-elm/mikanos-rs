use alloc::collections::LinkedList;

use linked_list_allocator::LockedHeap;
use uefi::table::boot::MemoryMapIter;

use kernel_lib::allocator::bitmap_memory_allocator::BitmapFrameAllocator;
use kernel_lib::allocator::memory_map::frame_range::FrameRange;
use kernel_lib::error::{KernelError, KernelResult};
use kernel_lib::error::AllocateReason::InitializeGlobalAllocator;

#[global_allocator]
static mut HEAP: BitmapFrameAllocator<FrameRange<'static>> = BitmapFrameAllocator::uninit();

pub fn init_alloc(memory_map: MemoryMapIter<'static>) -> KernelResult {
    unsafe {
        let memory_map_range = FrameRange::new(memory_map).ok_or(KernelError::FailedAllocate(InitializeGlobalAllocator))?;
        HEAP.init_heap(memory_map_range)
    }
}

// static mut MEMORY_POOL: [u8; 4096 * 32] = [0; 4096 * 32];
// #[global_allocator]
// static mut HEAP: LockedHeap = LockedHeap::empty();
//
// pub fn init_alloc() {
//     unsafe {
//         HEAP.lock()
//             .init(MEMORY_POOL.as_mut_ptr(), MEMORY_POOL.len());
//     }
// }