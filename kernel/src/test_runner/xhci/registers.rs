use crate::test_runner::xhci::{operational_registers, registers};
use pci::VolatileAccessible;

mod capability_registers;
mod operational_registers;

mod runtime_registers;

#[test_case]
fn it_new_all_registers() {
    registers();
}

#[test_case]
fn it_reset_host_controller() {
    execute_reset_host_controller();
    assert!(operational_registers().usb_sts().hch().read_flag_volatile());
}

pub(crate) fn execute_reset_host_controller() {
    operational_registers().reset_host_controller();
}
