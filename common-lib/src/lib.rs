#![cfg_attr(not(test), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod assembly;
pub mod frame_buffer;
pub mod math;
pub mod nums;
pub mod queue;
pub mod rectangle;
pub mod unit;
pub mod physical_address;


