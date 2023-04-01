use crate::allocate::bump_pointer_alloc::BumpPointerAlloc;

pub mod bump_pointer_alloc;

#[global_allocator]
static HEAP: BumpPointerAlloc = BumpPointerAlloc::new(0x2000_0100, 0x2000_0200);
