#![feature(option_as_slice)]
#![feature(type_alias_impl_trait)]
#![feature(strict_provenance)]
#![feature(once_cell)]
#![feature(sync_unsafe_cell)]


extern crate alloc;

pub mod error;
pub mod gop;
pub mod interrupt;
pub mod paging;
pub mod segment;
pub mod serial;
pub mod stack;
