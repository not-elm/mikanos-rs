use core::alloc::{GlobalAlloc, Layout};

use linked_list_allocator::LockedHeap;
use uefi::table::boot::MemoryMapIter;

use common_lib::math::Align;
use kernel_lib::allocator::MAX_MEMORY_SIZE;
use kernel_lib::allocator::memory_map::frame_iter::FrameIter;
use kernel_lib::error::{KernelError, KernelResult};
use kernel_lib::error::AllocateReason::InitializeGlobalAllocator;
use kernel_lib::sync::preemptive_mutex::PreemptiveMutex;

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
static HEAP: GlobalAllocator = GlobalAllocator::uninit();

pub fn init_alloc(memory_map: MemoryMapIter<'static>) -> KernelResult {
    unsafe {
        let mut memory_map_range = FrameIter::new(memory_map)
            .ok_or(KernelError::FailedAllocate(InitializeGlobalAllocator))?;

        let frame = memory_map_range
            .next()
            .unwrap();

        HEAP
            .0
            .lock()
            .lock()
            .init(frame.base_phys_addr().raw().align_up(64).unwrap() as *mut u8, MAX_MEMORY_SIZE);
    }

    Ok(())
}


struct GlobalAllocator(PreemptiveMutex<LockedHeap>);

impl GlobalAllocator {
    pub const fn uninit() -> Self {
        Self(PreemptiveMutex::new(LockedHeap::empty()))
    }
}


unsafe impl GlobalAlloc for GlobalAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self
            .0
            .lock()
            .alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self
            .0
            .lock()
            .dealloc(ptr, layout)
    }
}