use crate::xhci::registers::capability_registers::runtime_register_space_offset::RuntimeRegisterSpaceOffset;
use crate::xhci::registers::memory_mapped_addr::MemoryMappedAddr;

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
pub struct RuntimeRegisters {}

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
