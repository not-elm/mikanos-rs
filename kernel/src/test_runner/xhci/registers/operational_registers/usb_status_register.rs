use pci::xhc::registers::operational_registers::usb_status_register::host_controller_halted::HostControllerHalted;
use pci::xhc::registers::operational_registers::usb_status_register::usb_status_register_field::UsbStatusRegisterField;

use crate::test_runner::xhci::registers::execute_reset_host_controller;
use crate::test_runner::xhci::usb_status_register_offset;

#[test_case]
fn it_valid_hc_halted() {
    execute_reset_host_controller();
    assert!(HostControllerHalted::new_check_flag_true(usb_status_register_offset()).is_ok());
}
