use core::marker::PhantomData;

use macros::VolatileBits;

use crate::error::{InvalidRegisterReason, PciError, PciResult};
use crate::xhci::bit_zero_mask_lower_for;
use crate::xhci::registers::capability_registers::capability_length::CapabilityLength;
use crate::xhci::registers::capability_registers::capability_registers_field::CapabilityRegistersField;
use crate::xhci::registers::memory_mapped_addr::MemoryMappedAddr;
use crate::xhci::registers::operational_registers::operation_registers_offset::OperationalRegistersOffset;

/// RTS OFF
///
/// MemoryMappedAddress + RTS OFFでRuntime Registerのオフセットを表します。
///
/// # Offset
///
/// 0x18 Bytes
///
/// # Size
///
/// 4 Bytes
///
/// [Xhci Document] : 388 Page
///
/// [Xhci Document]: https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/extensible-host-controler-interface-usb-xhci.pdf
#[derive(VolatileBits)]
#[volatile_type(u32)]
#[offset_bit(0x18 * 8)]
pub struct RuntimeRegisterSpaceOffset(usize, PhantomData<MemoryMappedAddr>);

impl RuntimeRegisterSpaceOffset {
    pub fn new_with_check_size(
        mmio_addr: MemoryMappedAddr,
        cap_length: &CapabilityLength,
    ) -> PciResult<Self> {
        // MemoryMappedAddr + CAP LENGTHのオフセットにはOperationalRegistersが来るはずのため、
        // 少なくともこれ以上のサイズは必要
        let operation_registers_offset = OperationalRegistersOffset::new(mmio_addr, cap_length);
        let me = RuntimeRegisterSpaceOffset::new(mmio_addr);

        if me.read_rts_offset() <= operation_registers_offset.offset() as u32 {
            Err(PciError::InvalidRegister(
                InvalidRegisterReason::InvalidAddress {
                    specified_address: me.0,
                },
            ))
        } else {
            Ok(me)
        }
    }

    pub fn read_rts_offset(&self) -> u32 {
        bit_zero_mask_lower_for(5, self.read_volatile() as usize) as u32
    }
}
