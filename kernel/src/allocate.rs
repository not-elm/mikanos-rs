use uefi::table::boot::MemoryMapIter;

use kernel_lib::allocator::bitmap_memory_manager::BitmapAllocator;
use kernel_lib::allocator::memory_map_range::MemoryMapRange;
use kernel_lib::error::KernelResult;

pub mod bump_pointer_alloc;

static mut MEMORY_POOL: [u8; 4096 * 32] = [0; 4096 * 32];
#[global_allocator]
static mut HEAP: BitmapAllocator<MemoryMapRange> = BitmapAllocator::uninit();

pub fn init_alloc(memory_map: MemoryMapIter<'static>) -> KernelResult {
    unsafe { HEAP.init(MemoryMapRange::new(memory_map)) }
}
// #[global_allocator]
// static mut HEAP: LockedHeap = LockedHeap::empty();
//
// pub fn init_alloc(memory_map: MemoryMapIter<'static>) -> KernelResult {
//     unsafe {
//         HEAP.lock()
//             .init(MEMORY_POOL.as_mut_ptr(), MEMORY_POOL.len());
//     }
//
//     Ok(())
// }
