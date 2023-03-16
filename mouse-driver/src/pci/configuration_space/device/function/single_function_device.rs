use kernel_lib::error::{KernelError, KernelResult};

use crate::pci::configuration_space::device::header_type::general_header::GeneralHeader;
use crate::pci::configuration_space::device::header_type::pci_to_pci_bride_header::PciToPciBridgeHeader;

#[derive(Debug)]
pub enum SingleFunctionDevice {
    General(GeneralHeader),
    PciToPciBride(PciToPciBridgeHeader),
}

impl SingleFunctionDevice {
    pub fn expect_general(self) -> KernelResult<GeneralHeader> {
        if let Self::General(general) = self {
            Ok(general)
        } else {
            Err(KernelError::NotGeneralHeader)
        }
    }
}
