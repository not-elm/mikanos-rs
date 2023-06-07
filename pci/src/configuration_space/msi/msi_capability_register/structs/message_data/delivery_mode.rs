use crate::error::PciResult;
use crate::pci_bail;

// TODO
#[repr(u8)]
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub enum DeliveryMode {
    /// 指定された割り込み番号全てに対し、割り込み処理を行います。
    Fixed = 0b000,
    LowestPriority = 0b001,
    Smi = 0b010,
    Nmt = 0b100,
    Init = 0b101,
    ExtInt = 0b111,
}


impl DeliveryMode {
    pub fn new(raw: u8) -> PciResult<Self> {
        match raw {
            0b000 => Ok(Self::Fixed),
            0b001 => Ok(Self::LowestPriority),
            0b010 => Ok(Self::Smi),
            0b100 => Ok(Self::Nmt),
            0b101 => Ok(Self::Init),
            0b111 => Ok(Self::ExtInt),
            _ => pci_bail!("Delivery Mode Illegal value = {raw}")
        }
    }
}
