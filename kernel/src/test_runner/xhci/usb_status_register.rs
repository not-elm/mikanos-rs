use pci::xhci::registers::operational_registers::usb_status_register::controller_not_ready::ControllerNotReady;
use pci::xhci::registers::operational_registers::usb_status_register::host_controller_halted::HostControllerHalted;

use crate::test_runner::xhci::usb_status_register_offset;

#[test_case]
fn it_hc_halted() {
    //
    assert!(HostControllerHalted::new(usb_status_register_offset()).is_ok());
}

#[test_case]
fn it_controller_not_ready() {
    // コントローラーはデフォルトではReadyではない
    assert!(ControllerNotReady::new(usb_status_register_offset()).is_ok());
}
