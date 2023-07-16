#![feature(pointer_byte_offsets)]
#![cfg_attr(not(test), no_std)]
#[cfg(feature = "alloc")]
extern crate alloc;

pub mod array;
pub mod assembly;
pub mod elf;
pub mod error;
pub mod frame_buffer;
pub mod iter;
pub mod loader;
pub mod math;
pub mod nums;
pub mod physical_address;
pub mod queue;
pub mod repeat;
pub mod transform;
