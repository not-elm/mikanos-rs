use crate::error::PciResult;
use crate::xhci::registers::operational_registers::usb_status_register::controller_not_ready::ControllerNotReady;
use crate::xhci::registers::operational_registers::usb_status_register::host_controller_halted::HostControllerHalted;
use crate::xhci::registers::operational_registers::usb_status_register::usb_status_register_field::UsbStatusRegisterField;
use crate::xhci::registers::operational_registers::usb_status_register::usb_status_register_offset::UsbStatusRegisterOffset;

pub mod controller_not_ready;
pub mod host_controller_halted;
pub mod usb_status_register_field;
pub mod usb_status_register_offset;

#[derive(Debug, Clone)]
pub struct UsbStatusRegister {
    pub hch: HostControllerHalted,
    pub cnr: ControllerNotReady,
}

impl UsbStatusRegister {
    pub fn new(offset: UsbStatusRegisterOffset) -> PciResult<Self> {
        Ok(Self {
            hch: HostControllerHalted::new_check_flag_true(offset)?,
            cnr: ControllerNotReady::new(offset),
        })
    }
}
