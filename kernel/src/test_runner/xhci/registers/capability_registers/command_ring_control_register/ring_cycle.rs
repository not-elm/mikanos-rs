use pci::xhc::registers::operational_registers::command_ring_control_register::crcr_field::CrcrField;
use pci::xhc::registers::operational_registers::command_ring_control_register::ring_cycle_state::RingCycleState;

use crate::test_runner::xhci::command_ring_control_register_offset;

#[test_case]
fn it_access_ring_cycle() {
    assert!(RingCycleState::new_check_flag_false(command_ring_control_register_offset()).is_ok());
}
