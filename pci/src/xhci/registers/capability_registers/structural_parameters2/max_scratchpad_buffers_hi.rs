use core::marker::PhantomData;

use macros::VolatileBits;

use crate::xhci::registers::capability_registers::structural_parameters2::StructuralParameters2Offset;

/// Max Scratchpad Bufs Hi
///
/// # Offset
///
/// 21 Bits
///
/// # Size
///
/// 5 Bits
///
/// # Attribute
/// RO
///
///
/// [Xhci Document] : 384 Page
///
/// [Xhci Document]: https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/extensible-host-controler-interface-usb-xhci.pdf
#[derive(VolatileBits)]
#[volatile_type(u32)]
#[bits(5)]
#[offset_bit(21)]
pub struct MaxScratchPadBuffersHi(usize, PhantomData<StructuralParameters2Offset>);
