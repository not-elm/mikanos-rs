use common_lib::math::unit::{gib, kib};

pub mod allocate_map;
pub mod bitmap_memory_allocator;
pub mod bitmap_memory_manager;
pub mod memory_map;


pub const FRAME_SIZE: usize = kib(4);

pub const MAX_MEMORY_SIZE: usize = gib(10);
