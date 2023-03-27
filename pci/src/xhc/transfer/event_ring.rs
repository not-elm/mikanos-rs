use crate::error::PciResult;
use crate::xhc::allocator::memory_allocatable::MemoryAllocatable;
use crate::xhc::transfer::event::trb::EventTrb;
use crate::xhc::transfer::ring::{Ring, RingBase};
use crate::xhc::transfer::trb_raw_data::TrbRawData;

/// TODO DEBUG 実装
#[derive(Debug)]
pub struct EventRing {
    ring: Ring,
}

impl EventRing {
    pub fn new_with_alloc(
        ring_size: usize,
        allocator: &mut impl MemoryAllocatable,
    ) -> PciResult<Self> {
        Ok(Self {
            ring: Ring::new_with_alloc(ring_size, allocator)?,
        })
    }
    pub fn new(event_ring_addr: u64, ring_size: usize) -> Self {
        let mut ring = Ring::new(event_ring_addr, ring_size);

        Self { ring }
    }
    pub fn pop_event_trb(&mut self) -> Option<EventTrb> {
        let trb_buff = self.ring.pop();

        let trb_raw_data = TrbRawData::try_from(trb_buff).ok()?;
        unsafe { EventTrb::new(trb_raw_data) }
    }
}

impl RingBase for EventRing {
    fn ring(&self) -> &Ring {
        &self.ring
    }

    fn ring_mut(&mut self) -> &mut Ring {
        &mut self.ring
    }
}
