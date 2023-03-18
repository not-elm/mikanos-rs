use pci::xhci::registers::operational_registers::command_ring_control_register::command_ring_pointer::CommandRingPointer;

use crate::test_runner::xhci::command_ring_control_register_offset;

#[test_case]
fn it_valid_command_ring_pointer() {
    assert!(CommandRingPointer::new(command_ring_control_register_offset()).is_ok());
}
