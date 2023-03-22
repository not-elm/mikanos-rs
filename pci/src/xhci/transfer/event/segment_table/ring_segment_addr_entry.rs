use core::marker::PhantomData;

use macros::VolatileBits;

use crate::error::{OperationReason, PciError, PciResult};
use crate::wait_update_64bits_register_for;
use crate::xhci::bit_mask_zeros_lower_for;
use crate::xhci::transfer::event::segment::RingSegmentsBaseAddr;
use crate::xhci::transfer::event::segment_table::SegmentTableAddr;

/// Ring Segment Base Address Hi And Lo
///
/// # Size
///
/// 64Bits(下位6Bitsは予約領域)
///
/// [Xhci Document] : 515 Page
///
/// [Xhci Document]: https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/extensible-host-controler-interface-usb-xhci.pdf
#[derive(VolatileBits)]
#[volatile_type(u64)]
pub struct RingSegmentsBaseAddrEntry(usize, PhantomData<SegmentTableAddr>);

impl RingSegmentsBaseAddrEntry {
    pub fn update_ring_segment_addr(&self, ring_segment_addr: &RingSegmentsBaseAddr) -> PciResult {
        let addr = bit_mask_zeros_lower_for(6, ring_segment_addr.addr()) as u64;
        self.write_volatile(addr);
        wait_update_64bits_register_for(10, addr, self)
    }

    pub fn read_base_addr(&self) -> u64 {
        bit_mask_zeros_lower_for(6, self.read_volatile() as usize) as u64
    }
}
