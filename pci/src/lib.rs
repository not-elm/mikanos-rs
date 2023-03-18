#![no_std]
#![feature(type_alias_impl_trait)]
#![feature(strict_provenance)]

use macros::declaration_volatile_accessible;

pub mod configuration_space;
pub mod error;
pub mod pci_device_searcher;
pub mod xhci;

declaration_volatile_accessible!();
