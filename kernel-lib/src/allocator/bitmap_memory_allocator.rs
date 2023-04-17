use core::alloc::{GlobalAlloc, Layout};
use core::cell::OnceCell;
use core::ptr::null_mut;

use spin::Mutex;

use common_lib::math::Align;
use common_lib::physical_address::PhysicalAddress;

use crate::allocator::bitmap_memory_manager::BitmapMemoryFrameManager;
use crate::allocator::memory_map::frame_iterable::MemoryMapFrameIterable;
use crate::allocator::memory_map::frame_range::FrameRange;
use crate::error::{AllocateReason, KernelError, KernelResult};

pub struct BitmapFrameAllocator(OnceCell<Mutex<Allocator>>);


unsafe impl Sync for BitmapFrameAllocator {}

impl BitmapFrameAllocator {
    pub const fn uninit() -> BitmapFrameAllocator {
        Self(OnceCell::new())
    }


    pub fn init_heap(
        &mut self,
        frames: usize,
        memory_map: impl MemoryMapFrameIterable,
    ) -> KernelResult {
        let mut manager = BitmapMemoryFrameManager::new(memory_map)?;

        let frame_range = manager
            .allocate_frames(frames)
            .unwrap();

        let addr = frame_range
            .base()
            .base_phys_addr()
            .raw();

        self.0
            .set(Mutex::new(Allocator { frame_range, addr }))
            .map_err(|_| KernelError::FailedAllocate(AllocateReason::InitializeGlobalAllocator))
    }
}


unsafe impl GlobalAlloc for BitmapFrameAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut allocator = self.0.get().unwrap().lock();

        if allocator
            .align_up(layout.align())
            .is_err()
        {
            return null_mut();
        }

        if allocator.is_not_allocatable(layout.size()) {
            return null_mut();
        }

        let ptr = allocator.addr as *mut u8;
        allocator.addr += layout.size() as u64;

        ptr
    }


    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        if !ptr.is_null() {
            core::ptr::write_bytes(ptr, 0, layout.size());
            core::ptr::drop_in_place(ptr);
        }
    }
}


struct Allocator {
    frame_range: FrameRange,
    addr: u64,
}

impl Allocator {
    pub fn align_up(&mut self, align: usize) -> KernelResult {
        self.addr = self
            .addr
            .align_up(align)
            .ok_or(KernelError::NumSizeOver)?;
        Ok(())
    }


    pub fn is_not_allocatable(&self, bytes: usize) -> bool {
        !self.is_allocatable(bytes)
    }


    pub fn is_allocatable(&self, bytes: usize) -> bool {
        self.frame_range
            .is_contain_address(PhysicalAddress::new(self.addr + bytes as u64))
    }
}
