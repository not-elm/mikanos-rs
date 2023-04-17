use uefi::table::boot::MemoryMapIter;

use kernel_lib::allocator::bitmap_memory_allocator::BitmapFrameAllocator;
use kernel_lib::allocator::memory_map_range::MemoryMapRange;
use kernel_lib::error::KernelResult;

#[global_allocator]
static mut HEAP: BitmapFrameAllocator<MemoryMapRange<'static>> = BitmapFrameAllocator::uninit();

pub fn init_alloc(memory_map: MemoryMapIter<'static>) -> KernelResult {
    unsafe {
        let memory_map_range = MemoryMapRange::new(memory_map);
        HEAP.init_heap(memory_map_range)
    }
}
