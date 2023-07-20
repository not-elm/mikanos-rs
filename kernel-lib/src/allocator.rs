use core::cell::OnceCell;

use common_lib::math::unit::{gib, kib};

use crate::allocator::memory_map::frame_iter::FrameIter;

pub mod allocate_map;
pub mod bitmap_memory_allocator;
pub mod bitmap_memory_manager;
pub mod memory_map;


pub const FRAME_SIZE: usize = kib(4);

pub const MAX_MEMORY_SIZE: usize = gib(10);


pub static mut FRAME_ITER: GlobalFrameIter = GlobalFrameIter(OnceCell::new());


pub struct GlobalFrameIter(pub OnceCell<FrameIter<'static>>);




