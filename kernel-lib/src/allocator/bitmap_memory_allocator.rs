use core::alloc::{GlobalAlloc, Layout};
use core::cell::OnceCell;
use core::ptr::null_mut;

use spin::Mutex;

use common_lib::math::Align;
use common_lib::physical_address::PhysicalAddress;

use crate::allocator::bitmap_memory_manager::BitmapMemoryFrameManager;
use crate::allocator::memory_map::frame_iterable::MemoryMapFrameIterable;
use crate::error::{AllocateReason, KernelError, KernelResult};
use crate::serial_println;

pub struct BitmapFrameAllocator<MemoryMap>(OnceCell<Mutex<BitmapMemoryFrameManager<MemoryMap>>>)
    where
        MemoryMap: MemoryMapFrameIterable;

unsafe impl<MemoryMap> Sync for BitmapFrameAllocator<MemoryMap> where
    MemoryMap: MemoryMapFrameIterable
{}

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
        let mut manager = self.0.get().unwrap().lock();
        let phys_addr = manager.allocate_frame(layout);

        phys_addr
            .and_then(|phys_addr| {
                phys_addr
                    .raw()
                    .align_up(layout.align())
            })
            .map(|address| address as *mut u8)
            .inspect(|address| {
                serial_println!("PTR {}", core::ptr::read_volatile(*address));
                core::ptr::write_bytes(*address, 0, layout.size());
            })
            .unwrap()
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        serial_println!("FREE");
        let phs_addr = PhysicalAddress::new(ptr as u64);
        serial_println!("FREE Ptr {}", ptr.is_null());
        if !ptr.is_null() {
            core::ptr::write_bytes(ptr, 0, layout.size());
            core::ptr::drop_in_place(ptr);
        }

        let mut manager = self.0.get().unwrap().lock();
        manager
            .memory_map_mut()
            .frame_contains_address(phs_addr)
            .and_then(|frame| manager.free_frames(frame.id(), layout).ok())
            .unwrap();
    }
}
