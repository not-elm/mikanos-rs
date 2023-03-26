use crate::mmio_base_addr;
use pci::xhc::registers::capability_registers::capability_length::CapabilityLength;
use pci::xhc::registers::capability_registers::runtime_register_space_offset::RuntimeRegisterSpaceOffset;

#[test_case]
fn it_access_correct_rts_off() {
    RuntimeRegisterSpaceOffset::new_with_check_size(
        mmio_base_addr(),
        &CapabilityLength::new_check_length(mmio_base_addr()).unwrap(),
    )
    .unwrap();
}
