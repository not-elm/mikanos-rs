use pci::xhc::registers::internal::operational_registers::command_ring_control_register::command_ring_running::CommandRingRunning;
use pci::xhc::registers::internal::operational_registers::usb_command_register::run_stop::RunStop;
use pci::xhc::registers::internal::operational_registers::usb_command_register::usb_command_register_field::UsbCommandRegisterField;
use crate::test_runner::xhci::{command_ring_control_register_offset, operation_registers_offset};

#[test_case]
fn it_access_command_ring_running() {
    CommandRingRunning::new_with_check(
        command_ring_control_register_offset(),
        &RunStop::new(operation_registers_offset()),
    )
    .unwrap();
}
