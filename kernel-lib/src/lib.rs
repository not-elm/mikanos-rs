#![feature(option_as_slice)]
#![feature(type_alias_impl_trait)]
#![feature(strict_provenance)]
#![no_std]
#[cfg(test)]
extern crate alloc;

pub mod error;
pub mod gop;
pub mod stack;
