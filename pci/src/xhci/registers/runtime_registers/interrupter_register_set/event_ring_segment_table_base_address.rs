use crate::xhci::bitmask_lower_for;
use crate::xhci::registers::runtime_registers::interrupter_register_set::InterrupterRegisterSetOffset;
use core::marker::PhantomData;
use macros::VolatileBits;

/// ERSTBA
///
/// # Offset
///
/// InterrupterRegisterSetOffset + 0x30 Bytes
///
/// # Size
///
/// 64 Bits
///
/// # Attribute
/// RW
///
/// # Description
///
/// EventRingSegmentTableの先頭アドレスを保持します。
///
/// [Xhci Document] : 428 Page
///
/// [Xhci Document]: https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/extensible-host-controler-interface-usb-xhci.pdf
#[derive(VolatileBits)]
#[volatile_type(u64)]
#[offset(0x30 * 8)]
pub struct EventRingSegmentTableBaseAddress(usize, PhantomData<InterrupterRegisterSetOffset>);

impl EventRingSegmentTableBaseAddress {
    pub fn event_ring_segment_table_addr(&self) -> u64 {
        bitmask_lower_for(6, self.read_volatile() as usize) as u64
    }
}
