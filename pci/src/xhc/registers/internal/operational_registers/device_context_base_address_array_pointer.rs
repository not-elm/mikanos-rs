use macros::VolatileBits;

use crate::xhc::registers::internal::operational_registers::operation_registers_offset::OperationalRegistersOffset;

///DCBAAP
///
/// デバイスコンテキストを要素とする配列の先頭アドレスを保有します。
///
/// 配列の作成と、アドレスとのセットはソフトウェア側が定義する必要があります。
///
/// このレジスタの参照時となるメモリの構造は物理的に連続し、64Byteにアラインされている必要があります。
#[derive(VolatileBits)]
#[volatile_type(u64)]
#[offset_bit(6)]
pub struct DeviceContextBaseAddressArrayPointer(usize);

impl DeviceContextBaseAddressArrayPointer {
    pub fn new(offset: DeviceContextBaseAddressArrayPointerOffset) -> Self {
        Self(offset.offset())
    }
    pub fn update_device_context_array_addr(&self, addr: u64) {
        self.write_volatile(addr >> 6);
    }

    pub fn read_device_context_array_addr(&self) -> u64 {
        self.read_volatile() << 6
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
