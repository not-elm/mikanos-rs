use crate::error::PciResult;
use crate::xhc::registers::operational_registers::usb_status_register::controller_not_ready::ControllerNotReady;
use crate::xhc::registers::operational_registers::usb_status_register::host_controller_error::HostControllerError;
use crate::xhc::registers::operational_registers::usb_status_register::host_controller_halted::HostControllerHalted;
use crate::xhc::registers::operational_registers::usb_status_register::usb_status_register_field::UsbStatusRegisterField;
use crate::xhc::registers::operational_registers::usb_status_register::usb_status_register_offset::UsbStatusRegisterOffset;

pub mod controller_not_ready;
mod host_controller_error;
pub mod host_controller_halted;
pub mod usb_status_register_field;
pub mod usb_status_register_offset;

#[derive(Debug, Clone)]
pub struct UsbStatusRegister {
    hch: HostControllerHalted,
    cnr: ControllerNotReady,
    host_controller_error: HostControllerError,
}

impl UsbStatusRegister {
    pub fn new(offset: UsbStatusRegisterOffset) -> PciResult<Self> {
        // HCHは初期化時にリセットすればいいため、ここで1をチェックする必要はない?
        Ok(Self {
            hch: HostControllerHalted::new(offset),
            cnr: ControllerNotReady::new(offset),
            host_controller_error: HostControllerError::new(offset),
        })
    }

    pub fn hch(&self) -> &HostControllerHalted {
        &self.hch
    }

    pub fn cnr(&self) -> &ControllerNotReady {
        &self.cnr
    }

    pub fn host_controller_error(&self) -> &HostControllerError {
        &self.host_controller_error
    }
}
