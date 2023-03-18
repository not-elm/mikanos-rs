use pci::xhci::registers::operational_registers::usb_status_register::controller_not_ready::ControllerNotReady;
use pci::xhci::registers::operational_registers::usb_status_register::host_controller_halted::HostControllerHalted;
use pci::xhci::registers::operational_registers::usb_status_register::usb_status_register_field::UsbStatusRegisterField;

use crate::test_runner::xhci::initialize::execute_reset_host_controller;
use crate::test_runner::xhci::usb_status_register_offset;

#[test_case]
fn it_valid_hc_halted() {
    execute_reset_host_controller().unwrap();
    assert!(HostControllerHalted::new_check_flag_true(usb_status_register_offset()).is_ok());
}

// #[test_case]
// fn it_valid_controller_not_ready() {
//     execute_reset_host_controller().unwrap();
//     // コントローラーはデフォルトではReadyではない
//     assert!(ControllerNotReady::new_check_flag_true(usb_status_register_offset()).is_ok());
// }
