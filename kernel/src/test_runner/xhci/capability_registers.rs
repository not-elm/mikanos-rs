use mouse_driver::xhci::registers::capability_registers::CapabilityRegisters;

use crate::test_runner::xhci::mmio_base_addr;

#[test_case]
fn it_fetch_mmio_base_addr() {
    assert_eq!(0x8_00_00_00_00, mmio_base_addr())
}

#[test_case]
fn it_capability_len_is_greeter_than_zero() {
    assert!(
        0 < CapabilityRegisters::new(mmio_base_addr())
            .cap_length
            .read_volatile()
    );
}

#[test_case]
fn it_hcs_params1_is_not_null() {
    assert!(
        0 < CapabilityRegisters::new(mmio_base_addr())
            .hcs_params1
            .read_volatile()
    );
}

#[test_case]
fn it_hcs_params2_is_not_null() {
    assert!(
        0 < CapabilityRegisters::new(mmio_base_addr())
            .hcs_params2
            .read_volatile()
    );
}

#[test_case]
fn it_hcs_params3_is_not_null() {
    assert!(
        0 < CapabilityRegisters::new(mmio_base_addr())
            .hcs_params3
            .read_volatile()
    );
}

#[test_case]
fn it_hcc_params1_is_not_null() {
    assert!(
        0 < CapabilityRegisters::new(mmio_base_addr())
            .hcc_params1
            .read_volatile()
    );
}

#[test_case]
fn it_db_off_is_greeter_than_zero() {
    assert!(
        0 < CapabilityRegisters::new(mmio_base_addr())
            .db_off
            .read_volatile()
    );
}

#[test_case]
fn it_rts_off_is_greeter_than_zero() {
    assert!(
        0 < CapabilityRegisters::new(mmio_base_addr())
            .rts_off
            .read_volatile()
    );
}
