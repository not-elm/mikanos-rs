use crate::error::PciResult;
use crate::pci_bail;

#[repr(u8)]
#[derive(Debug, Eq, PartialEq, Copy, Clone, Ord, PartialOrd)]
pub enum CapabilityId {
    Msi = 0x5,
    MsiX = 0x11,
}


impl CapabilityId {
    pub fn try_from_u8(v: u8) -> PciResult<Self> {
        match v {
            0x05 => Ok(CapabilityId::Msi),
            0x11 => Ok(CapabilityId::MsiX),
            _ => pci_bail!("CapabilityId Illegal value = {v}"),
        }
    }
}
