use pci::xhci::registers::operational_registers::command_ring_control_register::ring_cycle_state::RingCycleState;

use crate::test_runner::xhci::command_ring_control_register_offset;

#[test_case]
fn it_access_ring_cycle() {
    assert!(RingCycleState::new(command_ring_control_register_offset()).is_ok());
}
