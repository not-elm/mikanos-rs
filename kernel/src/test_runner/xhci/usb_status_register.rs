use kernel_lib::println;
use pci::xhci::registers::operation_registers::usb_command_register::host_controller_reset::HostControllerReset;
use pci::xhci::registers::operation_registers::usb_command_register::host_system_error_enable::HostSystemErrorEnable;
use pci::xhci::registers::operation_registers::usb_status_register::host_controller_halted::HostControllerHalted;

use crate::test_runner::xhci::{operation_registers_offset, usb_status_register_offset};

#[test_case]
fn it_hc_halted() {
    let hc_halted = HostControllerHalted::new(usb_status_register_offset());
    println!(
        "{}",
        HostSystemErrorEnable::new(operation_registers_offset()).read_volatile()
    );
    assert!(hc_halted.read_volatile());

    let host_controller_reset = HostControllerReset::new(operation_registers_offset());

    host_controller_reset.reset();

    assert!(hc_halted.read_volatile());
}
