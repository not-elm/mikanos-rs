use alloc::rc::Rc;
use core::cell::RefCell;

use crate::error::PciResult;
use crate::xhc::allocator::memory_allocatable::MemoryAllocatable;
use crate::xhc::transfer::event::event_ring::EventRing;
use crate::xhc::transfer::event::event_ring_segment_table::EventRingSegmentTable;

pub trait InterrupterSetRegisterAccessible {
    fn write_event_ring_dequeue_pointer_at(
        &mut self,
        index: usize,
        event_ring_segment_addr: u64,
    ) -> PciResult;
    fn write_event_ring_segment_table_pointer_at(
        &mut self,
        index: usize,
        event_ring_segment_table_addr: u64,
    ) -> PciResult;

    fn write_interrupter_enable_at(&mut self, index: usize, is_enable: bool) -> PciResult;
    fn write_interrupter_pending_at(&mut self, index: usize, is_pending: bool) -> PciResult;

    fn read_dequeue_pointer_addr_at(&mut self, index: usize) -> u64;
    fn write_event_ring_segment_table_size(&mut self, index: usize, size: u16) -> PciResult;
}

pub(crate) fn setup_event_ring<T>(
    registers: &mut Rc<RefCell<T>>,
    event_ring_segment_table_size: u16,
    event_ring_segment_size: usize,
    allocator: &mut impl MemoryAllocatable,
) -> PciResult<(EventRingSegmentTable, EventRing<T>)>
where
    T: InterrupterSetRegisterAccessible,
{
    let event_ring_segment_table_addr =
        allocator.try_allocate_trb_ring(event_ring_segment_table_size as usize)?;
    let event_ring_segment_addr = allocator.try_allocate_trb_ring(event_ring_segment_size)?;
    registers
        .borrow_mut()
        .write_event_ring_segment_table_size(0, event_ring_segment_table_size)?;

    registers
        .borrow_mut()
        .write_event_ring_dequeue_pointer_at(0, event_ring_segment_addr)?;
    let event_ring_table = EventRingSegmentTable::new(
        event_ring_segment_table_addr,
        event_ring_segment_addr,
        event_ring_segment_size,
    )?;

    registers
        .borrow_mut()
        .write_event_ring_segment_table_pointer_at(0, event_ring_segment_table_addr)?;
    registers
        .borrow_mut()
        .write_interrupter_pending_at(0, true)?;
    registers
        .borrow_mut()
        .write_interrupter_enable_at(0, true)?;
    let event_ring = EventRing::new(event_ring_segment_addr, 32, registers);
    Ok((event_ring_table, event_ring))
}
