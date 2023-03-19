use crate::error::PciResult;
use crate::xhci::registers::capability_registers::capability_length::CapabilityLength;
use crate::xhci::registers::memory_mapped_addr::MemoryMappedAddr;

pub mod capability_length;
pub mod capability_parameters1;
pub mod capability_parameters2;
pub mod capability_registers_field;
pub mod doorbell_offset;
pub mod hci_version;
pub mod runtime_register_space_offset;
pub mod structural_parameters1;
pub mod structural_parameters2;
pub mod structural_parameters3;

#[derive(Debug)]
pub struct CapabilityRegisters {
    cap_length: CapabilityLength,
}

impl CapabilityRegisters {
    pub fn new(mmio_addr: MemoryMappedAddr) -> PciResult<Self> {
        Ok(Self {
            cap_length: CapabilityLength::new_check_length(mmio_addr)?,
        })
    }

    pub fn cap_length(&self) -> &CapabilityLength {
        &self.cap_length
    }
}
