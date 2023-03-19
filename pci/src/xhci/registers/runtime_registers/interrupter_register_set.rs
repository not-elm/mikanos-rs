use crate::xhci::registers::runtime_registers::RuntimeRegistersOffset;

pub mod event_ring_segment_table_base_address;
pub mod interrupter_management_register;
pub mod interrupter_register_set_field;

/// IRO
///
/// # Offset
///
/// Base(Primary) 0x20 Bytes
///
/// # Description
/// このレジスタはRunTimeRegistersの中に最大1024個配置でき、
/// 先頭の要素はPrimaryInterrupterと呼ばれます。
///
/// # Notes
///
/// * PrimaryInterrupterの中のレジスタ群はRunStopが1になるまえに初期化する必要があります。
///
/// * SecondaryInterrupters(恐らくPrimary以外を指す)はRunStopが1になった後でも初期化できますが、
/// 自身を対象にしたイベントが発行される前に初期化する必要があります。
///
/// [Xhci Document] : 424 Page
///
/// [Xhci Document]: https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/extensible-host-controler-interface-usb-xhci.pdf
pub struct InterrupterRegisterSet {}

/// # Address
///
/// RuntimeRegisterOffset + 0x20 Bytes
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct InterrupterRegisterSetOffset(usize);

impl InterrupterRegisterSetOffset {
    pub fn new(offset: RuntimeRegistersOffset, index: usize) -> Self {
        Self(offset.offset() + 0x20 + (index) * 32)
    }

    pub fn offset(&self) -> usize {
        self.0
    }
}
