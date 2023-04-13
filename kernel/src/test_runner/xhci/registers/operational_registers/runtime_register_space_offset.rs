use crate::test_runner::xhci::mmio_base_addr;
use pci::xhc::registers::internal::capability_registers::capability_length::CapabilityLength;
use pci::xhc::registers::internal::capability_registers::runtime_register_space_offset::RuntimeRegisterSpaceOffset;

#[test_case]
fn it_access_correct_rts_off() {
    RuntimeRegisterSpaceOffset::new_with_check_size(
        mmio_base_addr(),
        &CapabilityLength::new_check_length(mmio_base_addr()).unwrap(),
    )
    .unwrap();
}
