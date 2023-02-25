#![feature(pointer_byte_offsets)]
#![feature(provide_any)]
#![cfg_attr(not(test), no_std)]
#![feature(error_in_core)]

extern crate alloc;

pub mod elf;
pub mod error;
pub mod kernel;
