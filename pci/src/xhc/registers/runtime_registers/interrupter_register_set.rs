use core::fmt::Debug;

use kernel_lib::serial_println;

use crate::error::PciResult;
use crate::xhc::allocator::memory_allocatable::MemoryAllocatable;
use crate::xhc::registers::capability_registers::structural_parameters2::event_ring_segment_table_max::EventRingSegmentTableMax;
use crate::xhc::registers::runtime_registers::interrupter_register_set::event_ring_deque_pointer::EventRingDequeuePointer;
use crate::xhc::registers::runtime_registers::interrupter_register_set::event_ring_segment_table_base_address::EventRingSegmentTableBaseAddress;
use crate::xhc::registers::runtime_registers::interrupter_register_set::event_ring_segment_table_size::EventRingSegmentTableSize;
use crate::xhc::registers::runtime_registers::interrupter_register_set::interrupter_management_register::InterrupterManagementRegister;
use crate::xhc::registers::runtime_registers::interrupter_register_set::interrupter_register_set_field::InterrupterRegisterSetField;
use crate::xhc::registers::runtime_registers::RuntimeRegistersOffset;
use crate::xhc::transfer::event::event_ring::EventRingA;

pub mod event_ring_deque_pointer;
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
    event_ring_segment_table_size: EventRingSegmentTableSize,
    /// Offset: 0x10 Bytes
    erstba: EventRingSegmentTableBaseAddress,
    event_ring_dequeue_pointer: EventRingDequeuePointer,
}

impl InterrupterRegisterSet {
    pub fn new(offset: InterrupterRegisterSetOffset) -> Self {
        Self {
            iman: InterrupterManagementRegister::new(offset),
            event_ring_segment_table_size: EventRingSegmentTableSize::new(offset),
            erstba: EventRingSegmentTableBaseAddress::new(offset),
            event_ring_dequeue_pointer: EventRingDequeuePointer::new(offset),
        }
    }
    pub fn interrupter_management(&self) -> &InterrupterManagementRegister {
        &self.iman
    }

    pub fn event_ring_segment_table_size(&self) -> &EventRingSegmentTableSize {
        &self.event_ring_segment_table_size
    }

    pub fn event_ring_dequeue_pointer(&self) -> &EventRingDequeuePointer {
        &self.event_ring_dequeue_pointer
    }

    pub fn event_ring_table_max_size(&self) -> &EventRingSegmentTableSize {
        &self.event_ring_segment_table_size
    }

    pub fn event_ring_table_base_array_address(&self) -> &EventRingSegmentTableBaseAddress {
        &self.erstba
    }

    // TODO 削除
    pub fn setup_event_ring(
        &self,
        segment_table_entry_count: u16,
        erst_max: &EventRingSegmentTableMax,
        allocator: &mut impl MemoryAllocatable,
    ) -> PciResult<EventRingA> {
        self.event_ring_segment_table_size
            .update_event_ring_segment_table_size(erst_max, segment_table_entry_count)?;

        let event_ring = EventRingA::new(32, allocator)?;

        self.erstba.update_event_ring_segment_table_addr(
            event_ring.segment_table().segment_table_addr().addr() as u64,
        )?;
        self.event_ring_dequeue_pointer
            .update_deque_pointer(event_ring.segment_table().segments_base_addr().addr() as u64)?;
        serial_println!(
            "erdp {:x}",
            self.event_ring_dequeue_pointer.read_deque_pointer()
        );

        // self.iman.ie().write_flag_volatile(true);
        // self.iman.ip().write_flag_volatile(true);
        Ok(event_ring)
    }

    pub fn deque_ptr(&self) -> u64 {
        self.event_ring_dequeue_pointer.read_deque_pointer()
    }

    pub fn debug_trb(&self) {
        let ptr = self.event_ring_dequeue_pointer.read_deque_pointer();
        loop {
            serial_println!("{:x}", unsafe { *((ptr) as *mut u128) });
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
