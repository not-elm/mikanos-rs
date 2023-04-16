use common_lib::unit::{gib, kib};

pub mod allocate_map;
pub mod bitmap_memory_manager;
mod memory_map_frame_iterable;
pub mod memory_map_range;


pub(crate) const FRAME_SIZE: usize = kib(4);

pub(crate) const MAX_MEMORY_SIZE: usize = gib(128);
