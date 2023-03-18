use crate::configuration_space::device::header_type::general_header::GeneralHeader;
use crate::configuration_space::device::header_type::pci_to_pci_bride_header::PciToPciBridgeHeader;
use crate::error::{HeaderTypeReason, PciError, PciResult};

#[derive(Debug)]
pub enum SingleFunctionDevice {
    General(GeneralHeader),
    PciToPciBride(PciToPciBridgeHeader),
}

impl SingleFunctionDevice {
    pub fn expect_general(self) -> PciResult<GeneralHeader> {
        if let Self::General(general) = self {
            Ok(general)
        } else {
            Err(PciError::InvalidHeaderType(
                HeaderTypeReason::NotGeneralHeader,
            ))
        }
    }
}
