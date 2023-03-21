use crate::xhci::transfer::request_block::trb_template::cycle_bit::CycleBit;
use crate::xhci::transfer::request_block::trb_template::trb_type::TrbType;

pub mod cycle_bit;
mod trb_type;

/// TRB Template
///
/// # Size
///
/// 16 Bytes
///
/// # Description
///
/// xHCの転送要求を表すデータの基本構造を表します。
///
/// 全てのTRBは16バイトにアラインされている必要があります。
///
/// [Xhci Document] : 208, 465 Page
///
/// [Xhci Document]: https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/extensible-host-controler-interface-usb-xhci.pdf
pub struct TrbTemplate {
    cycle_bit: CycleBit,
    trb_type: TrbType,
}

#[derive(Debug)]
pub struct TrbAddr(usize);

impl TrbAddr {
    pub fn new(addr: usize) -> Self {
        Self(addr)
    }

    pub fn addr(&self) -> usize {
        self.0
    }
}
