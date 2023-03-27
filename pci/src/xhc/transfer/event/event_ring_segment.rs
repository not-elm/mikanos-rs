use crate::xhc::transfer::event::trb::EventTrb;
use crate::xhc::transfer::ring::{Ring, RingBase};
use crate::xhc::transfer::trb_raw_data::TrbRawData;

/// TODO DEBUG 実装
#[derive(Debug)]
pub struct EventRingSegment {
    ring: Ring,
}

impl EventRingSegment {
    pub fn new(event_ring_addr: u64, ring_size: usize) -> Self {
        let ring = Ring::new(event_ring_addr, ring_size);

        Self { ring }
    }
    pub fn pop_event_trb(&mut self) -> Option<EventTrb> {
        let trb_buff = self.ring.pop();

        let trb_raw_data = TrbRawData::try_from(trb_buff).ok()?;
        unsafe { EventTrb::new(trb_raw_data) }
    }
}

impl RingBase for EventRingSegment {
    fn ring(&self) -> &Ring {
        &self.ring
    }

    fn ring_mut(&mut self) -> &mut Ring {
        &mut self.ring
    }
}
