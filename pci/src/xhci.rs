use kernel_lib::println;

use crate::error::{PciError, PciResult};
use crate::xhci::registers::operational_registers::usb_command_register::host_controller_reset::HostControllerReset;
use crate::xhci::registers::operational_registers::usb_status_register::controller_not_ready::ControllerNotReady;
use crate::xhci::registers::operational_registers::usb_status_register::host_controller_halted::HostControllerHalted;

pub mod registers;

///
/// 1. xhcのリセット
/// 2. デバイスコンテキストの設定
pub fn _init() {}

pub fn _set_device_context() {}

pub fn reset_controller(
    hch: &HostControllerHalted,
    hcrst: &HostControllerReset,
    cnr: &ControllerNotReady,
) -> PciResult {
    if !hch.read_flag_volatile() {
        return Err(PciError::HostControllerNotHalted);
    }
    println!("start write true -> host controller reset");

    hcrst.reset();
    println!("write true -> host controller reset");

    cnr.wait_until_ready();
    println!(
        "controller is ready! current is = {}",
        cnr.read_flag_volatile()
    );

    Ok(())
}
