use core::marker::PhantomData;

use kernel_lib::println;
use macros::VolatileBits;

use crate::error::OperationReason::NotReflectedValue;
use crate::error::{PciError, PciResult};
use crate::xhc::transfer::event::segment::RingSegmentsBaseAddr;
use crate::xhc::transfer::event::segment_table::SegmentTableAddr;

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
#[offset_bit(6)]
pub struct EventRingAddressEntry(usize, PhantomData<SegmentTableAddr>);

impl EventRingAddressEntry {
    pub fn update_ring_segment_addr(&self, ring_segment_addr: &RingSegmentsBaseAddr) -> PciResult {
        let addr = (ring_segment_addr.addr()) as u64;
        self.write_volatile(addr);
        if addr == self.read_volatile() {
            Ok(())
        } else {
            println!("SEGMENTS BASE ADDR ENTRY");
            Err(PciError::FailedOperateToRegister(NotReflectedValue {
                expect: addr as usize,
                value: (self.read_volatile()) as usize,
            }))
        }
    }
}
