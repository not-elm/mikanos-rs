use crate::xhc::registers::operational_registers::operation_registers_offset::OperationalRegistersOffset;
use crate::xhc::registers::operational_registers::usb_command_register::controller_save_state::ControllerSaveState;
use crate::xhc::registers::operational_registers::usb_command_register::enable_wrap_event::EnableWrapEvent;
use crate::xhc::registers::operational_registers::usb_command_register::host_controller_reset::HostControllerReset;
use crate::xhc::registers::operational_registers::usb_command_register::host_system_error_enable::HostSystemErrorEnable;
use crate::xhc::registers::operational_registers::usb_command_register::interrupter_enable::InterrupterEnable;
use crate::xhc::registers::operational_registers::usb_command_register::light_host_controller_reset::LightHostControllerReset;
use crate::xhc::registers::operational_registers::usb_command_register::run_stop::RunStop;
use crate::xhc::registers::operational_registers::usb_command_register::usb_command_register_field::UsbCommandRegisterField;

pub mod controller_save_state;
pub mod host_controller_reset;
pub mod host_system_error_enable;

mod enable_wrap_event;
pub mod interrupter_enable;
pub mod light_host_controller_reset;
pub mod run_stop;
pub mod usb_command_register_field;

#[derive(Debug, Clone)]
pub struct UsbCommandRegister {
    run_stop: RunStop,
    hcrst: HostControllerReset,
    inte: InterrupterEnable,
    hsee: HostSystemErrorEnable,
    ewe: EnableWrapEvent,
    lhcrst: LightHostControllerReset,
    css: ControllerSaveState,
}

impl UsbCommandRegister {
    pub fn new(operational_offset: OperationalRegistersOffset) -> Self {
        Self {
            run_stop: RunStop::new(operational_offset),
            hcrst: HostControllerReset::new(operational_offset),
            inte: InterrupterEnable::new(operational_offset),
            hsee: HostSystemErrorEnable::new(operational_offset),
            lhcrst: LightHostControllerReset::new(operational_offset),
            ewe: EnableWrapEvent::new(operational_offset),
            css: ControllerSaveState::new(operational_offset),
        }
    }

    pub fn run_stop(&self) -> &RunStop {
        &self.run_stop
    }

    pub fn hcrst(&self) -> &HostControllerReset {
        &self.hcrst
    }

    pub fn inte(&self) -> &InterrupterEnable {
        &self.inte
    }

    pub fn hsee(&self) -> &HostSystemErrorEnable {
        &self.hsee
    }

    pub fn enable_wrap_event(&self) -> &EnableWrapEvent {
        &self.ewe
    }

    pub fn lhcrst(&self) -> &LightHostControllerReset {
        &self.lhcrst
    }

    pub fn css(&self) -> &ControllerSaveState {
        &self.css
    }
}
