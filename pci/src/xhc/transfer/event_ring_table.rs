use crate::error::PciResult;
use crate::xhc::allocator::memory_allocatable::MemoryAllocatable;
use crate::xhc::transfer::event::trb::EventTrb;
use crate::xhc::transfer::event_ring::EventRing;
use crate::xhc::transfer::ring::RingBase;

#[derive(Debug)]
pub struct EventRingTable {
    event_ring_table_addr: u64,
    event_ring: EventRing,
}

impl EventRingTable {
    pub fn new_with_alloc(allocator: &mut impl MemoryAllocatable) -> PciResult<Self> {
        let table_ptr_address = unsafe {
            allocator
                .try_allocate_with_align(core::mem::size_of::<u128>(), 64, 64 * 1024)?
                .address()?
        };
        let event_ring = EventRing::new_with_alloc(32, allocator)?;

        Ok(Self {
            event_ring_table_addr: table_ptr_address as u64,
            event_ring,
        })
    }
    pub fn table_address(&self) -> u64 {
        self.event_ring_table_addr
    }

    pub fn event_ring_address(&self) -> u64 {
        self.event_ring.ring_base_addr()
    }

    pub fn pop_event_trb(&mut self) -> Option<EventTrb> {
        self.event_ring.pop_event_trb()
    }
}
