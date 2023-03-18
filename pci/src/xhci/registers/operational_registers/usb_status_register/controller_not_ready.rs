use macros::VolatileBits;

use crate::error::PciResult;
use crate::xhci::registers::operational_registers::usb_status_register::usb_status_register_offset::UsbStatusRegisterOffset;

#[derive(VolatileBits)]
#[bits(1)]
#[offset(11)]
#[volatile_type(u32)]
pub struct ControllerNotReady(usize);

impl ControllerNotReady {
    pub fn new(offset: UsbStatusRegisterOffset) -> PciResult<Self> {
        let s = Self::new_uncheck(offset.offset());

        if s.read_flag_volatile() {
            Ok(s)
        } else {
            Ok(s)
            // Err(PciError::InvalidControllerNotReadyRegister)
        }
    }

    pub fn wait_until_ready(&self) {
        while self.read_flag_volatile() {}
    }
}
