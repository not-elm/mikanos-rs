use crate::configuration_space::device::header_type::general_header::GeneralHeader;
use crate::configuration_space::device::header_type::pci_to_pci_bride_header::PciToPciBridgeHeader;
use crate::error::PciResult;
use crate::pci_bail;

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
            pci_bail!("Not General Header")
        }
    }
}
