use crate::xhc::registers::internal::capability_registers::runtime_register_space_offset::RuntimeRegisterSpaceOffset;
use crate::xhc::registers::internal::memory_mapped_addr::MemoryMappedAddr;
use crate::xhc::registers::internal::runtime_registers::interrupter_register_set::{
    InterrupterRegisterSet, InterrupterRegisterSetOffset,
};

pub mod interrupter_register_set;

/// Host Controller Runtime Registers
///
///
/// # Address
///
/// MemoryMappedAddress + RTS OFF
///
/// # Notes
///
/// * このレジスタは通常DWord(32Bits)でアクセスする必要があります。
///
/// * QWord(64Bits)のアドレスが格納されたフィールドのアクセスには64Bitsのポインタか、
/// 32Bitsのポインタ二つを使用してアクセスする必要があります。
///
/// [Xhci Document] : 422 Page
///
/// [Xhci Document]: https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/extensible-host-controler-interface-usb-xhci.pdf
#[derive(Debug)]
pub struct RuntimeRegisters {
    interrupter_register_set: InterrupterRegisterSet,
}

impl RuntimeRegisters {
    pub fn new(offset: RuntimeRegistersOffset) -> Self {
        Self {
            interrupter_register_set: InterrupterRegisterSet::new(
                InterrupterRegisterSetOffset::new(offset, 0),
            ),
        }
    }

    pub fn interrupter_register_set(&self) -> &InterrupterRegisterSet {
        &self.interrupter_register_set
    }
}

/// # Address
///
/// MemoryMappedAddress + RTS OFF
pub struct RuntimeRegistersOffset(usize);

impl RuntimeRegistersOffset {
    pub fn new(mmio_addr: MemoryMappedAddr, rts_off: &RuntimeRegisterSpaceOffset) -> Self {
        Self(mmio_addr.addr() + rts_off.read_rts_offset() as usize)
    }

    pub fn offset(&self) -> usize {
        self.0
    }
}
