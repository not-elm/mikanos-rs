use linked_list_allocator::LockedHeap;
use uefi::table::boot::MemoryMapIter;

use common_lib::math::Align;
use kernel_lib::allocator::MAX_MEMORY_SIZE;
use kernel_lib::allocator::memory_map::frame_iter::FrameIter;
use kernel_lib::error::{KernelError, KernelResult};
use kernel_lib::error::AllocateReason::InitializeGlobalAllocator;

//
// #[global_allocator]
// static mut HEAP: BitmapFrameAllocator = BitmapFrameAllocator::uninit();
//
// pub fn init_alloc(memory_map: MemoryMapIter<'static>) -> KernelResult {
//     unsafe {
//         let memory_map_range = FrameIter::new(memory_map)
//             .ok_or(KernelError::FailedAllocate(InitializeGlobalAllocator))?;
//
//
//         const FRAMES: usize = MAX_FRAME_COUNT;
//         HEAP.init_heap(FRAMES, memory_map_range)
//     }
// }


// static mut MEMORY_POOL: [u8; MAX_MEMORY_SIZE] = [0; MAX_MEMORY_SIZE];
#[global_allocator]
static HEAP: LockedHeap = LockedHeap::empty();

pub fn init_alloc(memory_map: MemoryMapIter<'static>) -> KernelResult {
    unsafe {
        let mut memory_map_range = FrameIter::new(memory_map)
            .ok_or(KernelError::FailedAllocate(InitializeGlobalAllocator))?;

        let frame = memory_map_range
            .next()
            .unwrap();

        HEAP
            .lock()
            .init(frame.base_phys_addr().raw().align_up(64).unwrap() as *mut u8, MAX_MEMORY_SIZE);
    }

    Ok(())
}


