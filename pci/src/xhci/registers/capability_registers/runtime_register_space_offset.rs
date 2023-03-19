use core::marker::PhantomData;

use macros::VolatileBits;

use crate::error::{InvalidRegisterReason, PciError, PciResult};
use crate::xhci::registers::capability_registers::capability_length::CapabilityLength;
use crate::xhci::registers::capability_registers::capability_registers_field::CapabilityRegistersField;
use crate::xhci::registers::memory_mapped_addr::MemoryMappedAddr;
use crate::xhci::registers::operational_registers::operation_registers_offset::OperationalRegistersOffset;

/// RTS OFF
///
/// [Xhci Document](https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/extensible-host-controler-interface-usb-xhci.pdf) : 388 Page
///
/// MemoryMappedAddress + RTS OFFでRuntime Registerのオフセットを表します。
///
/// Offset: 0x18 Byte
///
/// Size: 4 Byte
#[derive(VolatileBits)]
#[volatile_type(u32)]
#[offset(0x18 * 8)]
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

    fn read_rts_offset(&self) -> u32 {
        shift_reserved_bits(self.read_volatile())
    }
}

fn shift_reserved_bits(bits: u32) -> u32 {
    // 下位5Bitsは予約領域
    bits >> 5
}

#[cfg(test)]
mod tests {
    use crate::xhci::registers::capability_registers::runtime_register_space_offset::shift_reserved_bits;

    #[test]
    fn it_read_rts_off_bits() {
        let addr = 0b1000_0000_0001_1111;
        assert_eq!(shift_reserved_bits(addr), 0b100_0000_0000)
    }
}
