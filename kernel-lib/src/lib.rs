#![feature(option_as_slice)]
#![feature(type_alias_impl_trait)]
#![feature(strict_provenance)]
#![feature(abi_x86_interrupt)]
#![feature(sync_unsafe_cell)]
#![feature(result_option_inspect)]
#![feature(slice_flatten)]
#![feature(naked_functions)]
#![cfg_attr(not(test), no_std)]
#![allow(clippy::identity_op)]
#![feature(thread_local)]
#![feature(asm_const)]
#![feature(atomic_bool_fetch_not)]


#[cfg(feature = "alloc")]
extern crate alloc;


pub mod acpi;
pub mod allocator;
pub mod apic;
pub mod context;
pub mod control_registers;
pub mod error;
pub mod fs;
pub mod gop;
pub mod interrupt;
pub mod io;
#[cfg(feature = "alloc")]
pub mod layers;
pub mod paging;
pub mod register;
pub mod segmentation;
pub mod serial;
pub mod sync;
pub mod task;
pub mod timer;


pub mod simple_fat {
    pub use simple_fat::*;
}


pub mod volatile_bits {
    pub use volatile_bits::*;
}
