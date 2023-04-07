use linked_list_allocator::LockedHeap;

pub mod bump_pointer_alloc;

static mut MEMORY_POOL: [u8; 4096 * 32] = [0; 4096 * 32];
#[global_allocator]
static mut HEAP: LockedHeap = LockedHeap::empty();

pub fn init_alloc() {
    unsafe {
        HEAP.lock()
            .init(MEMORY_POOL.as_mut_ptr(), MEMORY_POOL.len());
    }
}
