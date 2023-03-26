use pci::xhc::registers::operational_registers::OperationalRegisters;

use crate::test_runner::xhci::operation_registers_offset;
use crate::test_runner::xhci::registers::execute_reset_host_controller;

mod runtime_register_space_offset;
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
