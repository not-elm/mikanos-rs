#![no_std]
#![feature(type_alias_impl_trait)]
#![feature(strict_provenance)]

use macros::declaration_volatile_accessible;

use crate::error::{OperationReason, PciError, PciResult};

pub mod configuration_space;
pub mod error;
pub mod pci_device_searcher;
pub mod xhci;

declaration_volatile_accessible!();

pub(crate) fn wait_update_32bits_register_for<Addr, Offset>(
    wait_limit_count: usize,
    expect_value: u32,
    volatile: &impl VolatileAccessible<u32, Addr, Offset>,
) -> PciResult {
    // AsPrimitiveが使えないため、型別に宣言
    for _ in 0..wait_limit_count {
        if expect_value == volatile.read_volatile() {
            return Ok(());
        }
    }

    Err(PciError::FailedOperateToRegister(
        OperationReason::NotReflectedValue {
            value: expect_value as usize,
        },
    ))
}

pub(crate) fn wait_update_64bits_register_for<Addr, Offset>(
    wait_limit_count: usize,
    expect_value: u64,
    volatile: &impl VolatileAccessible<u64, Addr, Offset>,
) -> PciResult {
    for _ in 0..wait_limit_count {
        if expect_value == volatile.read_volatile() {
            return Ok(());
        }
    }

    Err(PciError::FailedOperateToRegister(
        OperationReason::NotReflectedValue {
            value: expect_value as usize,
        },
    ))
}
