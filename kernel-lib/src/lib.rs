#![feature(option_as_slice)]
#![feature(type_alias_impl_trait)]
#![feature(strict_provenance)]
#![feature(abi_x86_interrupt)]
#![feature(sync_unsafe_cell)]
#![feature(result_option_inspect)]
#![feature(slice_flatten)]
#![cfg_attr(not(test), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod acpi;
pub mod allocator;
pub mod apic;
pub mod control_registers;
pub mod error;
pub mod gop;
pub mod interrupt;
pub mod io;
#[cfg(feature = "alloc")]
pub mod layers;
pub mod paging;
pub mod register;
pub mod segmentation;
pub mod serial;
pub mod timer;

pub mod volatile_bits {
    pub use volatile_bits::*;
}
