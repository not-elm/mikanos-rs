use crate::error::PciResult;
use crate::xhc::allocator::memory_allocatable::MemoryAllocatable;
use crate::xhc::transfer::event::event_ring::EventRing;
use crate::xhc::transfer::event::event_ring_segment_table::EventRingSegmentTable;
use alloc::rc::Rc;
use core::cell::RefCell;

pub trait InterrupterSetRegisterAccessible {
    fn write_event_ring_dequeue_pointer(
        &mut self,
        index: usize,
        event_ring_segment_addr: u64,
    ) -> PciResult;
    fn write_event_ring_segment_table_pointer(
        &mut self,
        index: usize,
        event_ring_segment_table_addr: u64,
    ) -> PciResult;

    fn write_interrupter_enable(&mut self, index: usize, is_enable: bool) -> PciResult;
    fn write_interrupter_pending(&mut self, index: usize, is_pending: bool) -> PciResult;

    fn read_event_ring_addr(&self, index: usize) -> u64;
    fn write_event_ring_segment_table_size(&mut self, index: usize, size: u16) -> PciResult;
}
pub(crate) fn setup_event_ring<T>(
    registers: &Rc<RefCell<T>>,
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
        .write_event_ring_dequeue_pointer(0, event_ring_segment_addr)?;
    let event_ring_table = EventRingSegmentTable::new(
        event_ring_segment_table_addr,
        event_ring_segment_addr,
        event_ring_segment_size,
    )?;

    registers
        .borrow_mut()
        .write_event_ring_segment_table_pointer(0, event_ring_segment_table_addr)?;
    // self.write_interrupter_pending(0, true)?;
    // self.write_interrupter_enable(0, true)?;
    let event_ring = EventRing::new(event_ring_segment_addr, 32, registers);
    Ok((event_ring_table, event_ring))
}
