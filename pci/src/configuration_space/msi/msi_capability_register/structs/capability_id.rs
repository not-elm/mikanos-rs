use crate::error::{OldPciError, OldPciResult};

#[repr(u8)]
#[derive(Debug, Eq, PartialEq, Copy, Clone, Ord, PartialOrd)]
pub enum CapabilityId {
    Msi = 0x5,
    MsiX = 0x11,
}

impl CapabilityId {
    pub fn try_from_u8(v: u8) -> OldPciResult<Self> {
        match v {
            0x05 => Ok(CapabilityId::Msi),
            0x11 => Ok(CapabilityId::MsiX),
            _ => Err(OldPciError::IllegalEnumValue(v as usize)),
        }
    }
}
