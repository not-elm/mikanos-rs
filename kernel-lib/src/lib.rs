#![feature(option_as_slice)]
#![feature(type_alias_impl_trait)]
#![feature(strict_provenance)]
#![feature(abi_x86_interrupt)]
#![feature(sync_unsafe_cell)]
#![feature(result_option_inspect)]
#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

use macros::declaration_volatile_accessible;

declaration_volatile_accessible!();

pub mod allocator;
pub mod apic;
pub mod control_registers;
pub mod error;
pub mod gop;
pub mod interrupt;
pub mod paging;
pub mod register;
pub mod segmentation;
pub mod serial;
#[cfg(feature = "alloc")]
pub mod layers;
