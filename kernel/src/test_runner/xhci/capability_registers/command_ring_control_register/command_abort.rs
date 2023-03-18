use pci::xhci::registers::operational_registers::command_ring_control_register::command_abort::CommandAbort;

use crate::test_runner::xhci::command_ring_control_register_offset;

#[test_case]
fn it_access_command_abort() {
    assert!(CommandAbort::new(command_ring_control_register_offset()).is_ok());
}
