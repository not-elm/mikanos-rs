use core::fmt::Debug;

use crate::error::{AllocateReason, OperationReason, PciError, PciResult};
use crate::xhci::allocator::memory_allocatable::MemoryAllocatable;
use crate::xhci::registers::runtime_registers::interrupter_register_set::event_ring_segment_table_base_address::EventRingSegmentTableBaseAddress;
use crate::xhci::registers::runtime_registers::interrupter_register_set::event_ring_segment_table_size::EventRingSegmentTableSize;
use crate::xhci::registers::runtime_registers::interrupter_register_set::interrupter_management_register::InterrupterManagementRegister;
use crate::xhci::registers::runtime_registers::interrupter_register_set::interrupter_register_set_field::InterrupterRegisterSetField;
use crate::xhci::registers::runtime_registers::RuntimeRegistersOffset;

pub mod event_ring_segment_table_base_address;
pub mod event_ring_segment_table_size;
pub mod interrupter_management_register;
pub mod interrupter_register_set_field;

/// IRO
///
/// # Offset
///
/// Base(Primary) 0x20 Bytes
///
/// # Description
/// このレジスタはRunTimeRegistersの中に最大1024個配置でき、
/// 先頭の要素はPrimaryInterrupterと呼ばれます。
///
/// # Notes
///
/// * PrimaryInterrupterの中のレジスタ群はRunStopが1になるまえに初期化する必要があります。
///
/// * SecondaryInterrupters(恐らくPrimary以外を指す)はRunStopが1になった後でも初期化できますが、
/// 自身を対象にしたイベントが発行される前に初期化する必要があります。
///
/// [Xhci Document] : 424 Page
///
/// [Xhci Document]: https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/extensible-host-controler-interface-usb-xhci.pdf
pub struct InterrupterRegisterSet {
    /// Offset: 0
    iman: InterrupterManagementRegister,
    /// Offset: 0x08 Bytes
    erstsz: EventRingSegmentTableSize,
    /// Offset: 0x10 Bytes
    erstba: EventRingSegmentTableBaseAddress,
}

impl InterrupterRegisterSet {
    pub fn new(offset: InterrupterRegisterSetOffset) -> Self {
        Self {
            iman: InterrupterManagementRegister::new(offset),
            erstsz: EventRingSegmentTableSize::new(offset),
            erstba: EventRingSegmentTableBaseAddress::new(offset),
        }
    }

    pub fn iman(&self) -> &InterrupterManagementRegister {
        &self.iman
    }

    pub fn erstsz(&self) -> &EventRingSegmentTableSize {
        &self.erstsz
    }

    pub fn erstba(&self) -> &EventRingSegmentTableBaseAddress {
        &self.erstba
    }

    pub fn setup_event_ring(&self, allocator: &mut impl MemoryAllocatable) -> PciResult {
        let address = unsafe {
            allocator
                .allocate_with_align_64_bytes(4 * 4)
                .ok_or(PciError::FailedAllocate(AllocateReason::NotEnoughMemory))?
                .address()?
        };

        self.erstsz.update_event_ring_segment_table_size(1)?;
        todo!();
        self.erstba.update_event_ring_segment_table_addr(address);
        if self.erstba.event_ring_segment_table_addr() != 0 {
            Ok(())
        } else {
            Err(PciError::FailedOperateToRegister(
                OperationReason::NotReflectedValue { value: 1 },
            ))
        }
    }
}

/// # Address
///
/// RuntimeRegisterOffset + 0x20 Bytes
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct InterrupterRegisterSetOffset(usize);

impl InterrupterRegisterSetOffset {
    pub fn new(offset: RuntimeRegistersOffset, index: usize) -> Self {
        Self(offset.offset() + 0x20 + (index) * 32)
    }

    pub fn offset(&self) -> usize {
        self.0
    }
}
