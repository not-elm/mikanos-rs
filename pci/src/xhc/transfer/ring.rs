use xhci::ring::trb::Link;

use kernel_lib::{println, serial_println};

use crate::error::PciResult;
use crate::xhc::allocator::memory_allocatable::MemoryAllocatable;
use crate::xhc::transfer::event::trb::EventTrb;
use crate::xhc::transfer::trb_raw_data::TrbRawData;

pub trait RingBase {
    fn ring(&self) -> &Ring;
    fn ring_mut(&mut self) -> &mut Ring;
    fn ring_base_addr(&self) -> u64 {
        self.ring().ring_ptr_addr
    }
}

#[derive(Debug)]
pub struct Ring {
    ring_ptr_addr: u64,
    index: usize,
    ring_size: usize,
    cycle_bit: bool,
}

impl Ring {
    pub fn new(ring_ptr_addr: u64, ring_size: usize) -> Self {
        Self {
            ring_ptr_addr,
            index: 0,
            ring_size,
            cycle_bit: false,
        }
    }

    pub fn new_with_alloc(
        ring_size: usize,
        allocator: &mut impl MemoryAllocatable,
    ) -> PciResult<Self> {
        let bytes = core::mem::size_of::<u128>() * ring_size;

        let ring_ptr_addr =
            unsafe { allocator.try_allocate_with_align(bytes, 64, 64 * 1024) }?.address()?;

        Ok(Self::new(ring_ptr_addr as u64, ring_size))
    }
    pub fn pop(&mut self) -> [u32; 4] {
        let current_index = self.index;
        // if self.index < self.ring_size {
        //     self.index += 1;
        // } else {
        //     self.index = 0;
        //     self.cycle_bit = !self.cycle_bit;
        // }
        let data = self.ring_data_at(current_index);

        [data[0], data[1], data[2], data[3]]
    }
    pub fn push(&mut self, trb: impl Into<[u32; 4]>) {
        self.write_data(self.index, trb);
        self.index += 1;
        if self.index == self.ring_size - 1 {
            let mut link_trb = xhci::ring::trb::Link::new();
            link_trb.set_cycle_bit();
            self.write_data(self.index, link_trb);
            self.index = 0;
            self.cycle_bit = !self.cycle_bit;
        }
    }
    pub fn append_link_trb(&mut self) {
        let mut link_trb = Link::new();
        link_trb.set_cycle_bit();
        link_trb.set_toggle_cycle();
        link_trb.set_ring_segment_pointer(self.ring_ptr_addr);

        self.write_data(self.ring_size - 1, link_trb);
    }

    fn write_data(&mut self, index: usize, trb: impl Into<[u32; 4]>) {
        let cycle_bit = self.cycle_bit_as_u32();
        let current_data = self.ring_data_at(index);
        let write_data = trb.into();

        current_data[0] = write_data[0];
        current_data[1] = write_data[1];
        current_data[2] = write_data[2];
        current_data[3] = 0x1802;
    }

    fn ring_data_at(&mut self, index: usize) -> &mut [u32] {
        serial_println!("{:?}", unsafe {
            let raw = TrbRawData::new(*(self.ring_ptr().add(index))).unwrap();
            EventTrb::new(raw)
        });
        unsafe { core::slice::from_raw_parts_mut(self.ring_ptr().add(index).cast::<u32>(), 4) }
    }

    fn cycle_bit_as_u32(&self) -> u32 {
        if self.cycle_bit {
            1
        } else {
            0
        }
    }

    fn ring_ptr(&self) -> *mut u128 {
        self.ring_ptr_addr as *mut u128
    }
}
