use crate::VolatileAccessible;
use crate::xhci::registers::operational_registers::device_context_base_address_array_pointer::dcbaap_hi::DcbaapHi;
use crate::xhci::registers::operational_registers::device_context_base_address_array_pointer::dcbaap_lo::DcbaapLo;
use crate::xhci::registers::operational_registers::operation_registers_offset::OperationalRegistersOffset;

mod dcbaap_hi;
mod dcbaap_lo;

///DCBAAP
///
/// デバイスコンテキストを要素とする配列の先頭アドレスを保有します。
///
/// 配列の作成と、アドレスとのセットはソフトウェア側が定義する必要があります。
///
/// このレジスタの参照時となるメモリの構造は物理的に連続し、64Byteにアラインされている必要があります。
#[derive(Debug)]
pub struct DeviceContextBaseAddressArrayPointer {
    lo: DcbaapLo,
    hi: DcbaapHi,
}

impl DeviceContextBaseAddressArrayPointer {
    pub fn new(offset: DeviceContextBaseAddressArrayPointerOffset) -> Self {
        Self {
            lo: DcbaapLo::new(offset),
            hi: DcbaapHi::new(offset),
        }
    }

    pub fn write_volatile(&self, dcbaa_addr: u64) {
        self.lo.write_volatile((dcbaa_addr & 0xFF_FF) as u32);
        self.hi.write_volatile((dcbaa_addr >> 32) as u32);
    }

    pub fn read_volatile(&self) -> u64 {
        let lo = self.lo.read_volatile() as u64;
        let hi = self.hi.read_volatile() as u64;
        (hi << 32) | lo
    }
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct DeviceContextBaseAddressArrayPointerOffset(usize);

impl DeviceContextBaseAddressArrayPointerOffset {
    pub fn new(offset: OperationalRegistersOffset) -> Self {
        Self(offset.offset() + 0x30)
    }

    pub fn offset(&self) -> usize {
        self.0
    }
}
