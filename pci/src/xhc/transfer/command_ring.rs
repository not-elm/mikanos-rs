use crate::error::PciResult;
use crate::xhc::allocator::memory_allocatable::MemoryAllocatable;
use crate::xhc::transfer::ring::{Ring, RingBase};

#[derive(Debug)]
pub struct CommandRing {
    ring: Ring,
}

impl CommandRing {
    pub fn new_with_alloc(
        ring_size: usize,
        allocator: &mut impl MemoryAllocatable,
    ) -> PciResult<Self> {
        Ok(Self {
            ring: Ring::new_with_alloc(ring_size, allocator)?,
        })
    }

    pub fn new(ring_ptr_addr: u64, ring_size: usize) -> Self {
        let mut ring = Ring::new(ring_ptr_addr, ring_size);

        Self { ring }
    }

    pub fn enable_slot(&mut self) {
        self.ring.push(xhci::ring::trb::command::EnableSlot::new());
    }
}

impl RingBase for CommandRing {
    fn ring(&self) -> &Ring {
        &self.ring
    }

    fn ring_mut(&mut self) -> &mut Ring {
        &mut self.ring
    }
}
