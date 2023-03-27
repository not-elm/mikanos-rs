use core::marker::PhantomData;

use macros::VolatileBits;

use crate::error::{InvalidRegisterReason, PciError, PciResult};
use crate::xhc::registers::internal::capability_registers::structural_parameters2::structural_parameters2_field::StructuralParameters2Field;
use crate::xhc::registers::internal::capability_registers::structural_parameters2::StructuralParameters2Offset;

/// ERST Max
///
/// # Offset
///
/// StructuralParameters2Offset + 5 Bits
///
/// # Size
///
/// 5 Bits
///
/// # Attribute
/// RO
///
/// # Description
///
/// EventRingSegmentTableの最大サイズを表します。
///
/// EventRingSegmentTableの最大エントリーは2 ** ERST MAXになります。
///
/// 有効な値は0-15です。
///
/// [Xhci Document] : 428 Page
///
/// [Xhci Document]: https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/extensible-host-controler-interface-usb-xhci.pdf
#[derive(VolatileBits)]
#[volatile_type(u32)]
#[bits(4)]
#[offset_bit(4)]
pub struct EventRingSegmentTableMax(usize, PhantomData<StructuralParameters2Offset>);

impl EventRingSegmentTableMax {
    pub fn new_with_check_size(offset: StructuralParameters2Offset) -> PciResult<Self> {
        let me = Self::new(offset);
        if 15 < me.read_volatile() {
            Err(PciError::InvalidRegister(
                InvalidRegisterReason::InvalidAddress {
                    specified_address: offset.offset(),
                },
            ))
        } else {
            Ok(me)
        }
    }

    pub fn max_entries(&self) -> u32 {
        2u32.pow(self.read_volatile())
    }
}
