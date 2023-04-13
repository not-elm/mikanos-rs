#![feature(option_as_slice)]
#![feature(type_alias_impl_trait)]
#![feature(strict_provenance)]
#![feature(abi_x86_interrupt)]
#![feature(sync_unsafe_cell)]
#![no_std]

#[cfg(test)]
extern crate alloc;

use macros::declaration_volatile_accessible;


declaration_volatile_accessible!();

pub mod apic;
pub mod error;
pub mod gop;
pub mod interrupt;
pub mod paging;
pub mod segment;
pub mod serial;
pub mod stack;
