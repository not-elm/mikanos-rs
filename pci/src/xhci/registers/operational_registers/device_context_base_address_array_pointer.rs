use macros::VolatileBits;

use crate::xhci::registers::operational_registers::operation_registers_offset::OperationalRegistersOffset;

///DCBAAP
///
/// デバイスコンテキストを要素とする配列の先頭アドレスを保有します。
///
/// 配列の作成と、アドレスとのセットはソフトウェア側が定義する必要があります。
///
/// このレジスタの参照時となるメモリの構造は物理的に連続し、64Byteにアラインされている必要があります。
#[derive(VolatileBits)]
#[volatile_type(u64)]
#[bits(58)]
#[offset(6)]
pub struct DeviceContextBaseAddressArrayPointer(usize);

impl DeviceContextBaseAddressArrayPointer {
    pub fn new(offset: OperationalRegistersOffset) -> Self {
        Self::new_uncheck(offset.offset())
    }
}
