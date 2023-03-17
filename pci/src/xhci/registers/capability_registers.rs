use macros::Volatile;

use crate::xhci::registers::capability_registers::capability_length::CapabilityLength;
use crate::xhci::registers::capability_registers::capability_parameters1::CapabilityParameters1;
use crate::xhci::registers::capability_registers::capability_parameters2::CapabilityParameters2;
use crate::xhci::registers::capability_registers::doorbell_offset::DoorbellOffset;
use crate::xhci::registers::capability_registers::runtime_register_space_offset::RuntimeRegisterSpaceOffset;
use crate::xhci::registers::capability_registers::structural_parameters1::StructuralParameters1;
use crate::xhci::registers::capability_registers::structural_parameters2::StructuralParameters2;
use crate::xhci::registers::capability_registers::structural_parameters3::StructuralParameters3;

pub mod capability_length;
pub mod capability_parameters1;
pub mod capability_parameters2;
pub mod doorbell_offset;
pub mod runtime_register_space_offset;
pub mod structural_parameters1;
pub mod structural_parameters2;
pub mod structural_parameters3;

//
pub struct CapabilityRegisters {
    pub cap_length: CapabilityLength,
    pub hci_version: HciVersion,
    pub hcs_params1: StructuralParameters1,
    pub hcs_params2: StructuralParameters2,
    pub hcs_params3: StructuralParameters3,
    pub hcc_params1: CapabilityParameters1,
    pub db_off: DoorbellOffset,
    pub rts_off: RuntimeRegisterSpaceOffset,
    pub hcc_params2: CapabilityParameters2,
}

#[derive(Debug, Clone, Volatile)]
#[volatile_type(u16)]
pub struct HciVersion(usize);

// impl CapabilityRegisters {
//     pub fn new(mmio_base_addr: MemoryMappedAddr) -> PciResult<Self> {
//         let offset = |addr: usize| mmio_base_addr + addr;
//         let cap_length = CapabilityLength::new_with_check(offset(0)).unwrap();
//         let db_off =
//             DoorbellOffset::new_with_check(offset(0x14), cap_length.read_volatile()).unwrap();
//         let rts_off = RuntimeRegisterSpaceOffset::new(offset(0x18));
//
//         Ok(Self {
//             cap_length,
//             hci_version: HciVersion::new(offset(0x02)),
//             hcs_params1: StructuralParameters1::new(offset(0x04)),
//             hcs_params2: StructuralParameters2::new(offset(0x08)),
//             hcs_params3: StructuralParameters3::new(offset(0x0C)),
//             hcc_params1: CapabilityParameters1::new(offset(0x10)),
//             db_off,
//             rts_off,
//             hcc_params2: CapabilityParameters2::new(offset(0x1C)),
//         })
//     }
// }
