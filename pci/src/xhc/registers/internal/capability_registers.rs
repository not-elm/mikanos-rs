use crate::error::PciResult;
use crate::xhc::registers::internal::capability_registers::capability_length::CapabilityLength;
use crate::xhc::registers::internal::capability_registers::runtime_register_space_offset::RuntimeRegisterSpaceOffset;
use crate::xhc::registers::internal::capability_registers::structural_parameters1::{
    StructuralParameters1, StructuralParameters1Offset,
};
use crate::xhc::registers::internal::capability_registers::structural_parameters2::{
    StructuralParameters2, StructuralParameters2Offset,
};
use crate::xhc::registers::memory_mapped_addr::MemoryMappedAddr;

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

/// Address: MemoryMappedAddress
///
/// [Xhci Document](https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/extensible-host-controler-interface-usb-xhci.pdf)
/// : 381 Page
#[derive(Debug)]
pub struct CapabilityRegisters {
    /// Offset: 0
    cap_length: CapabilityLength,
    /// Offset: 0x04 Byte
    hcs_params1: StructuralParameters1,
    /// Offset: 0x08 Byte
    hcs_params2: StructuralParameters2,
    /// Offset: 0x18 Byte
    rts_off: RuntimeRegisterSpaceOffset,
}

impl CapabilityRegisters {
    pub fn new(mmio_addr: MemoryMappedAddr) -> PciResult<Self> {
        let cap_length = CapabilityLength::new_check_length(mmio_addr)?;
        let rts_off = RuntimeRegisterSpaceOffset::new_with_check_size(mmio_addr, &cap_length)?;
        Ok(Self {
            cap_length,
            hcs_params1: StructuralParameters1::new(StructuralParameters1Offset::new(mmio_addr)),
            hcs_params2: StructuralParameters2::new(StructuralParameters2Offset::new(mmio_addr))?,
            rts_off,
        })
    }

    pub fn cap_length(&self) -> &CapabilityLength {
        &self.cap_length
    }

    pub fn hcs_params1(&self) -> &StructuralParameters1 {
        &self.hcs_params1
    }

    pub fn hcs_params2(&self) -> &StructuralParameters2 {
        &self.hcs_params2
    }

    pub fn rts_off(&self) -> &RuntimeRegisterSpaceOffset {
        &self.rts_off
    }
}
