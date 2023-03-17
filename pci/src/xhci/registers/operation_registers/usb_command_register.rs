use crate::error::PciResult;
use crate::xhci::registers::operation_registers::usb_command_register::controller_save_state::ControllerSaveState;
use crate::xhci::registers::operation_registers::usb_command_register::host_controller_reset::HostControllerReset;
use crate::xhci::registers::operation_registers::usb_command_register::host_system_error_enable::HostSystemErrorEnable;
use crate::xhci::registers::operation_registers::usb_command_register::interrupter_enable::InterrupterEnable;
use crate::xhci::registers::operation_registers::usb_command_register::light_host_controller_reset::LightHostControllerReset;
use crate::xhci::registers::operation_registers::usb_command_register::run_stop::RunStop;

pub mod controller_save_state;
pub mod host_controller_reset;
pub mod host_system_error_enable;
pub mod interrupter_enable;
pub mod light_host_controller_reset;
pub mod run_stop;

pub struct UsbCommandRegister {
    pub run_stop: RunStop,
    pub hcr: HostControllerReset,
    pub inte: InterrupterEnable,
    pub hsee: HostSystemErrorEnable,
    pub lhcrst: LightHostControllerReset,
    pub css: ControllerSaveState,
}

impl UsbCommandRegister {
    pub fn new(mmio_base_addr: usize, cap_length: u8) -> PciResult<Self> {
        let base_offset = mmio_base_addr + cap_length as usize;
        let offset = |addr: usize| base_offset + addr;

        Ok(Self {
            run_stop: RunStop::new(base_offset),
            hcr: HostControllerReset::new(offset(1)),
            inte: InterrupterEnable::new(offset(2)),
            hsee: HostSystemErrorEnable::new(offset(3)),
            lhcrst: LightHostControllerReset::new(offset(7)),
            css: ControllerSaveState::new(offset(8)),
        })
    }
}
