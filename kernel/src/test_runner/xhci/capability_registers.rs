use pci::xhci::registers::capability_registers::CapabilityRegisters;

use crate::test_runner::xhci::mmio_base_addr;

#[test_case]
fn it_fetch_mmio_base_addr() {
    assert_eq!(0x8_00_00_00_00, mmio_base_addr())
}

#[test_case]
fn it_capability_len_is_greeter_than_zero() {
    assert!(CapabilityRegisters::new(mmio_base_addr()).is_ok());
}
