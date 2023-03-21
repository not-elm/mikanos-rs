use crate::xhci::registers::operational_registers::operation_registers_offset::OperationalRegistersOffset;

use macros::VolatileBits;

mod dcbaap_hi;
mod dcbaap_lo;

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
