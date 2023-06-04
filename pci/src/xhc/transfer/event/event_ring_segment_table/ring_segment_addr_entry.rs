use kernel_lib::volatile_bits::volatile_bits;

use crate::xhc::transfer::event::event_ring_segment_table::SegmentTableAddr;

/// Ring Segment Base Address Hi And Lo
///
/// # Size
///
/// 64Bits(下位6Bitsは予約領域)
///
/// [Xhci Document] : 515 Page
///
/// [Xhci Document]: https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/extensible-host-controler-interface-usb-xhci.pdf
#[volatile_bits(
type = u64,
)]
pub struct EventRingAddressEntry(SegmentTableAddr);
