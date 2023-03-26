use crate::error::PciResult;
use crate::xhc::registers::operational_registers::usb_status_register::controller_not_ready::ControllerNotReady;
use crate::xhc::registers::operational_registers::usb_status_register::host_controller_halted::HostControllerHalted;
use crate::xhc::registers::operational_registers::usb_status_register::usb_status_register_field::UsbStatusRegisterField;
use crate::xhc::registers::operational_registers::usb_status_register::usb_status_register_offset::UsbStatusRegisterOffset;

pub mod controller_not_ready;
pub mod host_controller_halted;
pub mod usb_status_register_field;
pub mod usb_status_register_offset;

#[derive(Debug, Clone)]
pub struct UsbStatusRegister {
    hch: HostControllerHalted,
    cnr: ControllerNotReady,
}

impl UsbStatusRegister {
    pub fn new(offset: UsbStatusRegisterOffset) -> PciResult<Self> {
        // HCHは初期化時にリセットすればいいため、ここで1をチェックする必要はない?
        Ok(Self {
            hch: HostControllerHalted::new(offset),
            cnr: ControllerNotReady::new(offset),
        })
    }

    pub fn hch(&self) -> &HostControllerHalted {
        &self.hch
    }

    pub fn cnr(&self) -> &ControllerNotReady {
        &self.cnr
    }
}
