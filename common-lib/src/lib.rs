#![cfg_attr(not(test), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod assembly;
pub mod frame_buffer;
pub mod math;
pub mod nums;
pub mod physical_address;
pub mod queue;
pub mod repeat;
