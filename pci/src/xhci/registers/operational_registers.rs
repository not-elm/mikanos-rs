use crate::error::PciResult;
use crate::VolatileAccessible;
use crate::xhci::registers::operational_registers::config_register::{ConfigRegister, ConfigRegisterOffset};
use crate::xhci::registers::operational_registers::device_context_base_address_array_pointer::{DeviceContextBaseAddressArrayPointer, DeviceContextBaseAddressArrayPointerOffset};
use crate::xhci::registers::operational_registers::operation_registers_offset::OperationalRegistersOffset;
use crate::xhci::registers::operational_registers::usb_command_register::host_controller_reset::HostControllerReset;
use crate::xhci::registers::operational_registers::usb_command_register::run_stop::RunStop;
use crate::xhci::registers::operational_registers::usb_command_register::UsbCommandRegister;
use crate::xhci::registers::operational_registers::usb_status_register::controller_not_ready::ControllerNotReady;
use crate::xhci::registers::operational_registers::usb_status_register::host_controller_halted::HostControllerHalted;
use crate::xhci::registers::operational_registers::usb_status_register::usb_status_register_offset::UsbStatusRegisterOffset;
use crate::xhci::registers::operational_registers::usb_status_register::UsbStatusRegister;

pub mod command_ring_control_register;
pub mod config_register;
pub mod device_context_base_address_array_pointer;
pub mod operation_registers_offset;
pub mod usb_command_register;
pub mod usb_status_register;

#[derive(Debug)]
pub struct OperationalRegisters {
    usb_command: UsbCommandRegister,
    usb_sts: UsbStatusRegister,
    dcbaap: DeviceContextBaseAddressArrayPointer,
    config: ConfigRegister,
}

impl OperationalRegisters {
    pub fn new(offset: OperationalRegistersOffset) -> PciResult<Self> {
        Ok(Self {
            usb_command: UsbCommandRegister::new(offset),
            usb_sts: UsbStatusRegister::new(UsbStatusRegisterOffset::new(offset))?,
            dcbaap: DeviceContextBaseAddressArrayPointer::new(
                DeviceContextBaseAddressArrayPointerOffset::new(offset),
            ),
            config: ConfigRegister::new(ConfigRegisterOffset::new(offset)),
        })
    }
    pub fn reset_host_controller(&self) {
        reset_controller(
            self.usb_sts().hch(),
            self.usb_command().run_stop(),
            self.usb_command().hcrst(),
            self.usb_sts().cnr(),
        )
    }
    pub fn run_host_controller(&self) {
        self.usb_command().run_stop().write_flag_volatile(true);
        self.usb_sts().hch().until_not_halted();
    }
    pub fn usb_command(&self) -> &UsbCommandRegister {
        &self.usb_command
    }

    pub fn usb_sts(&self) -> &UsbStatusRegister {
        &self.usb_sts
    }
    pub fn dcbaap(&self) -> &DeviceContextBaseAddressArrayPointer {
        &self.dcbaap
    }
    pub fn config(&self) -> &ConfigRegister {
        &self.config
    }
}

fn reset_controller(
    hch: &HostControllerHalted,
    run_stop: &RunStop,
    hcrst: &HostControllerReset,
    cnr: &ControllerNotReady,
) {
    if !hch.read_flag_volatile() {
        run_stop.write_flag_volatile(false);
    }
    hch.until_halted();

    hcrst.reset();

    cnr.wait_until_ready();
}
