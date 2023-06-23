use crate::error::PciResult;
use crate::pci_error;
use crate::xhc::allocator::memory_allocatable::MemoryAllocatable;
use crate::xhc::transfer::{trb_buffer_from_address, trb_byte_size};
use crate::xhc::transfer::trb_raw_data::TrbRawData;

#[derive(Debug, Copy, Clone)]
pub struct TransferRing {
    ring_ptr_base_address: u64,
    ring_ptr_address: u64,
    ring_end_address: u64,
    ring_size: usize,
    cycle_bit: bool,
}


impl TransferRing {
    pub fn new(ring_ptr_base_address: u64, ring_size: usize, cycle_bit: bool) -> Self {
        Self {
            ring_ptr_base_address,
            ring_ptr_address: ring_ptr_base_address,
            ring_end_address: ring_ptr_base_address + trb_byte_size() * (ring_size - 1) as u64,
            ring_size,
            cycle_bit,
        }
    }


    pub fn new_with_alloc(
        ring_size: usize,
        cycle_bit: bool,
        allocator: &mut impl MemoryAllocatable,
    ) -> PciResult<Self> {
        let ring_ptr_base_address = allocator.try_allocate_trb_ring(ring_size)?;
        Ok(Self::new(ring_ptr_base_address, ring_size, cycle_bit))
    }


    pub fn push(&mut self, trb: [u32; 4]) -> PciResult {
        self.write(trb)?;

        self.ring_ptr_address += trb_byte_size();
        if self.is_end_address(self.ring_ptr_address) {
            self.rollback()?;
        }
        Ok(())
    }


    pub fn read(&self) -> Option<TrbRawData> {
        self.read_transfer_request_block(self.ring_ptr_address)
    }


    pub fn ring_size(&self) -> usize {
        self.ring_size
    }


    pub fn read_transfer_request_block(&self, trb_addr: u64) -> Option<TrbRawData> {
        let ptr = trb_addr as *const u128;
        if ptr.is_null() {
            return None;
        }
        Some(TrbRawData::new_unchecked(unsafe { *(ptr) }))
    }


    pub fn base_address(&self) -> u64 {
        self.ring_ptr_base_address
    }


    pub fn toggle_cycle_bit(&mut self) {
        self.cycle_bit = !self.cycle_bit;
    }


    pub fn current_ptr_address(&self) -> u64 {
        self.ring_ptr_address
    }


    pub fn is_end_address(&self, address: u64) -> bool {
        self.ring_end_address <= address
    }


    pub fn is_end_event_address(&self, address: u64) -> bool {
        self.ring_end_address <= address
    }


    pub fn cycle_bit(&self) -> bool {
        self.cycle_bit
    }


    fn rollback(&mut self) -> PciResult {
        let mut link = xhci::ring::trb::Link::new();
        link.set_toggle_cycle();
        link.set_ring_segment_pointer(self.ring_ptr_base_address);

        self.write(link.into_raw())?;

        self.ring_ptr_address = self.ring_ptr_base_address;
        self.toggle_cycle_bit();
        Ok(())
    }


    fn write(&mut self, src_buff: [u32; 4]) -> PciResult {
        let dest_deref = unsafe {
            (self.ring_ptr_address as *mut u128)
                .as_mut()
                .ok_or(pci_error!("Failed operate transfer ring"))
        }?;

        let dest_buff = trb_buffer_from_address(dest_deref);

        dest_buff[0] = src_buff[0];
        dest_buff[1] = src_buff[1];
        dest_buff[2] = src_buff[2];
        dest_buff[3] = (src_buff[3] & !0b1) | self.cycle_bit_as_u32();

        Ok(())
    }


    fn cycle_bit_as_u32(&self) -> u32 {
        if self.cycle_bit {
            1
        } else {
            0
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::xhc::transfer::transfer_ring::TransferRing;
    use crate::xhc::transfer::trb_byte_size;
    use crate::xhc::transfer::trb_raw_data::TrbRawData;

    #[test]
    fn it_push_trb() {
        let buff = [0u128; 32];
        let mut ring = TransferRing::new(buff.as_ptr() as u64, 32, true);
        let enable_slot_trb =
            TrbRawData::from(xhci::ring::trb::command::EnableSlot::new().into_raw());
        let is_ok = ring
            .push(enable_slot_trb.into_u32_array())
            .is_ok();

        assert!(is_ok);
        let enable_slot_buff: [u32; 4] = enable_slot_trb.into();
        let buff = buff.as_ptr().cast::<u32>();
        unsafe {
            let buff = core::slice::from_raw_parts(buff, 4);
            assert_eq!(buff[0], enable_slot_buff[0]);
            assert_eq!(buff[1], enable_slot_buff[1]);
            assert_eq!(buff[2], enable_slot_buff[2]);
            assert_eq!(buff[3], enable_slot_buff[3] | 1);
            assert_eq!(
                ring.ring_ptr_address,
                ring.ring_ptr_base_address + trb_byte_size()
            )
        }
    }

    #[test]
    #[cfg(target_endian = "little")]
    fn it_push_link_trb_and_rollback() {
        let buff = [0u128; 2];
        let mut ring = TransferRing::new(buff.as_ptr() as u64, 2, true);
        let enable_slot_trb =
            TrbRawData::try_from(xhci::ring::trb::command::EnableSlot::new().into_raw()).unwrap();

        assert!(ring
            .push(enable_slot_trb.into_u32_array())
            .is_ok());

        let mut link = xhci::ring::trb::Link::new();
        link.set_toggle_cycle();
        let link_buff = link.into_raw();
        unsafe {
            let buff = buff
                .as_ptr()
                .add(1)
                .cast::<u32>();
            let buff = core::slice::from_raw_parts(buff, 4);
            assert_eq!(buff[0], (ring.ring_ptr_base_address & 0xFFFF_FFFF) as u32);
            assert_eq!(buff[1], (ring.ring_ptr_base_address >> 32) as u32);
            assert_eq!(buff[2], link_buff[2]);
            assert_eq!(buff[3], link_buff[3] | 1);
            assert_eq!(ring.ring_ptr_address, ring.ring_ptr_base_address);
            assert!(!ring.cycle_bit);
        }
    }
}
