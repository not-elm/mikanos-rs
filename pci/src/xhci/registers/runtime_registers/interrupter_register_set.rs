use core::fmt::Debug;

use kernel_lib::println;

use crate::error::{AllocateReason, PciError, PciResult};
use crate::VolatileAccessible;
use crate::xhci::allocator::memory_allocatable::MemoryAllocatable;
use crate::xhci::registers::capability_registers::structural_parameters2::event_ring_segment_table_max::EventRingSegmentTableMax;
use crate::xhci::registers::runtime_registers::interrupter_register_set::event_ring_deque_pointer::EventRingDequeuePointer;
use crate::xhci::registers::runtime_registers::interrupter_register_set::event_ring_segment_table_base_address::EventRingSegmentTableBaseAddress;
use crate::xhci::registers::runtime_registers::interrupter_register_set::event_ring_segment_table_size::EventRingSegmentTableSize;
use crate::xhci::registers::runtime_registers::interrupter_register_set::interrupter_management_register::InterrupterManagementRegister;
use crate::xhci::registers::runtime_registers::interrupter_register_set::interrupter_register_set_field::InterrupterRegisterSetField;
use crate::xhci::registers::runtime_registers::RuntimeRegistersOffset;
use crate::xhci::transfer::event::event_ring::EventRing;

mod event_ring_deque_pointer;
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
#[derive(Debug)]
pub struct InterrupterRegisterSet {
    /// Offset: 0
    iman: InterrupterManagementRegister,
    /// Offset: 0x08 Bytes
    erstsz: EventRingSegmentTableSize,
    /// Offset: 0x10 Bytes
    erstba: EventRingSegmentTableBaseAddress,
    erdp: EventRingDequeuePointer,
}

impl InterrupterRegisterSet {
    pub fn new(offset: InterrupterRegisterSetOffset) -> Self {
        Self {
            iman: InterrupterManagementRegister::new(offset),
            erstsz: EventRingSegmentTableSize::new(offset),
            erstba: EventRingSegmentTableBaseAddress::new(offset),
            erdp: EventRingDequeuePointer::new(offset),
        }
    }

    pub fn setup_event_ring(
        &self,
        segment_table_entry_count: u16,
        erst_max: &EventRingSegmentTableMax,
        allocator: &mut impl MemoryAllocatable,
    ) -> PciResult<EventRing> {
        self.erstsz
            .update_event_ring_segment_table_size(erst_max, segment_table_entry_count)?;

        let event_ring = EventRing::new(32, allocator)?;

        self.erstba.update_event_ring_segment_table_addr(
            event_ring.segment_table().segment_table_addr().addr(),
        )?;
        self.erdp.update_deque_pointer(
            event_ring
                .segment_table()
                .segments_base_addr()
                .read_volatile(),
        )?;
        //
        // self.iman.ie().write_flag_volatile(true);
        // self.iman.ip().write_flag_volatile(true);
        Ok(event_ring)
    }

    pub fn dp(&self) {
        let tb = ((self.erdp.read_volatile() << 4) as *mut u16);
        println!("{:x}", unsafe { *tb });
    }
}

fn allocate_event_ring_segment_table(
    segment_table_entry_count: u16,
    allocator: &mut impl MemoryAllocatable,
) -> PciResult<usize> {
    unsafe {
        allocator
            .allocate_with_align(
                core::mem::size_of::<u32>() * 4 * segment_table_entry_count as usize,
                64,
                64 * 1024,
            )
            .ok_or(PciError::FailedAllocate(AllocateReason::NotEnoughMemory))?
            .address()
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
