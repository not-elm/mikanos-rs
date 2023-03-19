pub mod interrupter_register_set;

/// Host Controller Runtime Registers
///
/// [Xhci Document](https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/extensible-host-controler-interface-usb-xhci.pdf)
/// : 422 Page
///
/// Address: MemoryMappedAddress + RTS OFF
///
/// Note: このレジスタは通常DWord(32Bits)でアクセスする必要があります。
///
/// Note: QWord(64Bits)のアドレスが格納されたフィールドのアクセスには64Bitsのポインタか、
/// 32Bitsのポインタ二つを使用してアクセスする必要があります。
pub struct RuntimeRegisters {}
