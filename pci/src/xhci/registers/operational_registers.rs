use crate::error::PciResult;
use crate::VolatileAccessible;
use crate::xhci::registers::operational_registers::operation_registers_offset::OperationalRegistersOffset;
use crate::xhci::registers::operational_registers::usb_command_register::UsbCommandRegister;
use crate::xhci::registers::operational_registers::usb_status_register::usb_status_register_offset::UsbStatusRegisterOffset;
use crate::xhci::registers::operational_registers::usb_status_register::UsbStatusRegister;
use crate::xhci::reset_controller;

pub mod command_ring_control_register;
pub mod config_register;
pub mod device_context_base_address_array_pointer;
pub mod operation_registers_offset;
pub mod usb_command_register;
pub mod usb_status_register;

#[derive(Debug, Clone)]
pub struct OperationRegisters {
    pub usb_command: UsbCommandRegister,
    pub usb_sts: UsbStatusRegister,
}

impl OperationRegisters {
    pub fn new(offset: OperationalRegistersOffset) -> PciResult<Self> {
        Ok(Self {
            usb_command: UsbCommandRegister::new(offset),
            usb_sts: UsbStatusRegister::new(UsbStatusRegisterOffset::new(offset))?,
        })
    }
    pub fn reset_host_controller(&self) -> PciResult {
        reset_controller(
            &self.usb_sts.hch,
            &self.usb_command.run_stop,
            &self.usb_command.hcrst,
            &self.usb_sts.cnr,
        )
    }
    pub fn run_host_controller(&self) {
        self.usb_command.run_stop.write_flag_volatile(true);
        self.usb_sts.hch.until_not_halted();
    }
}
