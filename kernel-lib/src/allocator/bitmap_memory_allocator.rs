use core::alloc::{GlobalAlloc, Layout};
use core::cell::OnceCell;
use core::ptr::null_mut;

use spin::Mutex;

use common_lib::math::Align;

use crate::allocator::bitmap_memory_manager::BitmapMemoryFrameManager;
use crate::allocator::memory_map_frame_iterable::MemoryMapFrameIterable;
use crate::allocator::FRAME_SIZE;
use crate::error::{AllocateReason, KernelError, KernelResult};
use crate::serial_println;

pub struct BitmapFrameAllocator<MemoryMap>(OnceCell<Mutex<BitmapMemoryFrameManager<MemoryMap>>>)
where
    MemoryMap: MemoryMapFrameIterable;

unsafe impl<MemoryMap> Sync for BitmapFrameAllocator<MemoryMap> where
    MemoryMap: MemoryMapFrameIterable
{
}

impl<MemoryMap> BitmapFrameAllocator<MemoryMap>
where
    MemoryMap: MemoryMapFrameIterable + Clone,
{
    pub const fn uninit() -> BitmapFrameAllocator<MemoryMap> {
        Self(OnceCell::new())
    }


    pub fn init_heap(&mut self, memory_map: MemoryMap) -> KernelResult {
        self.0
            .set(Mutex::new(BitmapMemoryFrameManager::new(memory_map)?))
            .map_err(|_| KernelError::FailedAllocate(AllocateReason::InitializeGlobalAllocator))
    }
}


unsafe impl<MemoryMap> GlobalAlloc for BitmapFrameAllocator<MemoryMap>
where
    MemoryMap: MemoryMapFrameIterable + Clone,
{
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        serial_println!("allocate layout {:?}", layout);
        let mut manager = self.0.get().unwrap().lock();
        let frame = manager.allocate_frame(layout);

        let addr = frame
            .and_then(|frame| {
                frame
                    .descriptor()
                    .phys_start
                    .align_up(layout.align())
            })
            .map(|address| address as *mut u8)
            .unwrap_or(null_mut());

        serial_println!("allocated  0x{:x}", addr as u64);
        addr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let phs_addr = ptr as u64;
        let mut manager = self.0.get().unwrap().lock();
        let frame = manager
            .memory_map_mut()
            .find(|frame| {
                let frame_addr = frame.descriptor().phys_start;
                frame_addr <= phs_addr && phs_addr <= (frame_addr + FRAME_SIZE as u64)
            })
            .unwrap();


        self.0
            .get()
            .unwrap()
            .lock()
            .free_frames(frame.id(), layout)
            .unwrap();
    }
}
