#![no_std]
#![feature(type_alias_impl_trait)]
#![feature(strict_provenance)]
#![feature(pointer_is_aligned)]
#![feature(pointer_byte_offsets)]
#![feature(maybe_uninit_slice)]
#![feature(ptr_as_uninit)]
#![feature(allocator_api)]
extern crate alloc;

use kernel_lib::volatile_bits::VolatileBitsReadable;

use crate::error::{OldPciError, OldPciResult, OperationReason};

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


#[allow(unused)]
pub(crate) fn wait_update_32bits_register_for(
    wait_limit_count: usize,
    expect_value: u32,
    volatile: &impl VolatileBitsReadable<u32>,
) -> OldPciResult {
    // AsPrimitiveが使えないため、型別に宣言
    for _ in 0..wait_limit_count {
        if expect_value == volatile.read_volatile() {
            return Ok(());
        }
    }

    Err(OldPciError::FailedOperateToRegister(
        OperationReason::NotReflectedValue {
            value: volatile.read_volatile() as usize,
            expect: expect_value as usize,
        },
    ))
}

#[allow(unused)]
pub(crate) fn wait_update_64bits_register_for(
    wait_limit_count: usize,
    expect_value: u64,
    volatile: &impl VolatileBitsReadable<u64>,
) -> OldPciResult {
    for _ in 0..wait_limit_count {
        if expect_value == volatile.read_volatile() {
            return Ok(());
        }
    }

    Err(OldPciError::FailedOperateToRegister(
        OperationReason::NotReflectedValue {
            expect: expect_value as usize,
            value: volatile.read_volatile() as usize,
        },
    ))
}
