use core::alloc::{GlobalAlloc, Layout};

use linked_list_allocator::LockedHeap;
use uefi::table::boot::MemoryMapIter;

use common_lib::math::Align;
use kernel_lib::allocator::memory_map::frame_iter::FrameIter;
use kernel_lib::allocator::MAX_MEMORY_SIZE;
use kernel_lib::error::AllocateReason::InitializeGlobalAllocator;
use kernel_lib::error::{KernelError, KernelResult};
use kernel_lib::interrupt;

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

        HEAP.0.lock().init(
            frame
                .base_phys_addr()
                .raw()
                .align_up(64)
                .unwrap() as *mut u8,
            MAX_MEMORY_SIZE,
        );
    }

    Ok(())
}


struct GlobalAllocator(LockedHeap);

impl GlobalAllocator {
    pub const fn uninit() -> Self {
        Self(LockedHeap::empty())
    }
}


unsafe impl GlobalAlloc for GlobalAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        interrupt::asm::with_free(|| self.0.alloc(layout))
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        interrupt::asm::with_free(|| self.0.dealloc(ptr, layout))
    }
}
