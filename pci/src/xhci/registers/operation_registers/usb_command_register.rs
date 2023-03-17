use crate::xhci::registers::operation_registers::usb_command_register::run_stop::RunStop;

pub mod host_controller_reset;
pub mod run_stop;

pub struct UsbCommandRegister {
    pub run_stop: RunStop,
}
