use macros::VolatileBits;

use crate::error::{InvalidRegisterReason, PciError, PciResult};
use crate::xhci::registers::operational_registers::usb_status_register::usb_status_register_offset::UsbStatusRegisterOffset;

#[derive(VolatileBits)]
#[bits(1)]
pub struct HostControllerHalted(usize);

impl HostControllerHalted {
    pub fn new(offset: UsbStatusRegisterOffset) -> PciResult<Self> {
        let s = HostControllerHalted::new_uncheck(offset.offset());
        if s.read_flag_volatile() {
            Ok(s)
        } else {
            Err(PciError::InvalidRegister(
                InvalidRegisterReason::HostControllerNotHalted,
            ))
        }
    }
}
