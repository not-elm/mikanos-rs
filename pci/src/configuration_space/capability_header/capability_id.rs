use crate::configuration_space::capability_header::capability_id::CapabilityId::{Msi, MsiX};
use crate::error::{PciError, PciResult};

#[repr(u8)]
#[derive(Debug, Eq, PartialEq, Copy, Clone, Ord, PartialOrd)]
pub enum CapabilityId {
    Msi = 0x5,
    MsiX = 0x11,
}

impl CapabilityId {
    pub fn try_from_u8(v: u8) -> PciResult<Self> {
        match v {
            0x05 => Ok(Msi),
            0x11 => Ok(MsiX),
            _ => Err(PciError::IllegalEnumValue(v as usize)),
        }
    }
}
