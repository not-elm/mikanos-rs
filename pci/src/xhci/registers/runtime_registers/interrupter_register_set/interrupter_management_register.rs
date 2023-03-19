pub mod interrupt_pending;

/// IMAN
///
/// # Offset
///
/// RuntimeRegistersOffset + 0x20 Bytes
///
/// # Size
///
/// 32 Bits
///
/// # Description
///
/// xHCの割り込みの有効または、無効化の操作(IE)と、
/// 割り込みの発生状態の検知(IP)ができます。
///
/// [Xhci Document] : 425 Page
///
/// [Xhci Document]: https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/extensible-host-controler-interface-usb-xhci.pdf
pub struct InterrupterManagementRegister {}
