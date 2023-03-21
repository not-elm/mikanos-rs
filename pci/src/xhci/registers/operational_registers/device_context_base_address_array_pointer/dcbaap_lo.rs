use macros::VolatileBits;

use crate::xhci::registers::operational_registers::device_context_base_address_array_pointer::DeviceContextBaseAddressArrayPointerOffset;

///DCBAAP
///
/// デバイスコンテキストを要素とする配列の先頭アドレス(下位32Bit)を保有します。
///
/// 配列の作成と、アドレスとのセットはソフトウェア側が定義する必要があります。
///
/// このレジスタの参照時となるメモリの構造は物理的に連続し、64Byteにアラインされている必要があります。
#[derive(VolatileBits)]
#[volatile_type(u32)]
#[offset_bit(6)]
pub struct DcbaapLo(usize);

impl DcbaapLo {
    pub fn new(offset: DeviceContextBaseAddressArrayPointerOffset) -> Self {
        Self::new_uncheck(offset.offset())
    }

    pub fn read_addr_lo(&self) -> u32 {
        self.read_volatile() << 6
    }
}
