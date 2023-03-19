use pci::xhci::registers::operational_registers::OperationalRegisters;

use crate::test_runner::xhci::initialize::execute_reset_host_controller;
use crate::test_runner::xhci::operation_registers_offset;

mod usb_command_register;
mod usb_status_register;

#[test_case]
fn it_valid_operational_registers() {
    execute_reset_host_controller();
    OperationalRegisters::new(operation_registers_offset()).unwrap();
}

#[test_case]
fn it_start_running_host_controller() {
    execute_reset_host_controller();
    OperationalRegisters::new(operation_registers_offset())
        .unwrap()
        .run_host_controller();
    assert!(true);
}
