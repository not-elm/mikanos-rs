use pci::xhc::registers::internal::operational_registers::command_ring_control_register::command_stop::CommandStop;
use pci::xhc::registers::internal::operational_registers::command_ring_control_register::crcr_field::CommandRingControlRegisterField;
use crate::test_runner::xhci::command_ring_control_register_offset;

#[test_case]
fn it_access_command_stop() {
    assert!(CommandStop::new_check_flag_false(command_ring_control_register_offset()).is_ok());
}
