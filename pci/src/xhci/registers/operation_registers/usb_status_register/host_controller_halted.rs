use macros::VolatileBits;

use crate::error::{PciError, PciResult};
use crate::xhci::registers::operation_registers::usb_status_register::usb_status_register_offset::UsbStatusRegisterOffset;

#[derive(VolatileBits)]
pub struct HostControllerHalted(usize);

impl HostControllerHalted {
    pub fn new(offset: UsbStatusRegisterOffset) -> Self {
        Self::new_uncheck(offset.offset())
    }

    pub fn new_expect_halted(offset: UsbStatusRegisterOffset) -> PciResult<Self> {
        let s = HostControllerHalted::new_uncheck(offset.offset());
        if s.read_flag_volatile() {
            Ok(s)
        } else {
            Err(PciError::HostControllerNotHalted)
        }
    }
}
