/// MMIO BASE ADDRESS
///
/// [XHCIドキュメント](https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/extensible-host-controler-interface-usb-xhci.pdf)
/// ではBASEと称されます。
///
/// 64Bitsの環境の場合、PCIコンフィグレーション空間のBAR0とBAR1から次のように計算して求めます。
/// (BAR1 << 32) | (BAR0 | 0xFF_FF_FF_F0)
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct MemoryMappedAddr(usize);

impl MemoryMappedAddr {
    pub(crate) fn new(mmio_addr: usize) -> Self {
        Self(mmio_addr)
    }

    pub fn addr(&self) -> usize {
        self.0
    }
}
