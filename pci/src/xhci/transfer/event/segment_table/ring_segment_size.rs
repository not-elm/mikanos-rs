use core::marker::PhantomData;

use macros::VolatileBits;

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
#[add_addr_bytes(0x08)]
#[bits(16)]
pub struct RingSegmentSize(usize, PhantomData<SegmentTableAddr>);
