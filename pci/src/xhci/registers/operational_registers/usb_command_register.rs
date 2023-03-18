use crate::xhci::registers::operational_registers::operation_registers_offset::OperationalRegistersOffset;
use crate::xhci::registers::operational_registers::usb_command_register::controller_save_state::ControllerSaveState;
use crate::xhci::registers::operational_registers::usb_command_register::host_controller_reset::HostControllerReset;
use crate::xhci::registers::operational_registers::usb_command_register::host_system_error_enable::HostSystemErrorEnable;
use crate::xhci::registers::operational_registers::usb_command_register::interrupter_enable::InterrupterEnable;
use crate::xhci::registers::operational_registers::usb_command_register::light_host_controller_reset::LightHostControllerReset;
use crate::xhci::registers::operational_registers::usb_command_register::run_stop::RunStop;
use crate::xhci::registers::operational_registers::usb_command_register::usb_command_register_field::UsbCommandRegisterField;

pub mod controller_save_state;
pub mod host_controller_reset;
pub mod host_system_error_enable;

pub mod interrupter_enable;
pub mod light_host_controller_reset;
pub mod run_stop;
pub mod usb_command_register_field;

#[derive(Debug, Clone)]
pub struct UsbCommandRegister {
    pub run_stop: RunStop,
    pub hcr: HostControllerReset,
    pub inte: InterrupterEnable,
    pub hsee: HostSystemErrorEnable,
    pub lhcrst: LightHostControllerReset,
    pub css: ControllerSaveState,
}

impl UsbCommandRegister {
    pub fn new(operational_offset: OperationalRegistersOffset) -> Self {
        Self {
            run_stop: RunStop::new(operational_offset),
            hcr: HostControllerReset::new(operational_offset),
            inte: InterrupterEnable::new(operational_offset),
            hsee: HostSystemErrorEnable::new(operational_offset),
            lhcrst: LightHostControllerReset::new(operational_offset),
            css: ControllerSaveState::new(operational_offset),
        }
    }
}
