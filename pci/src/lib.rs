#![no_std]
#![feature(type_alias_impl_trait)]
#![feature(strict_provenance)]
#![feature(pointer_is_aligned)]
#![feature(pointer_byte_offsets)]
#![feature(maybe_uninit_slice)]
#![feature(ptr_as_uninit)]
#![feature(allocator_api)]
extern crate alloc;


pub mod class_driver;
pub mod configuration_space;
pub mod error;
pub mod pci_device_searcher;
pub mod xhc;


pub(crate) fn flag_to_num(flag: bool) -> usize {
    if flag {
        1
    } else {
        0
    }
}

