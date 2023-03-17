use macros::VolatileFlag;

use crate::error::{PciError, PciResult};
use crate::xhci::registers::operation_registers::usb_status_register::usb_status_register_offset::UsbStatusRegisterOffset;

#[derive(VolatileFlag)]
pub struct HCHalted(usize);

impl HCHalted {
    pub fn new_expect_halted(offset: UsbStatusRegisterOffset) -> PciResult<Self> {
        HCHalted::new_expect_to_be(true, offset.offset()).ok_or(PciError::HostControllerNotHalted)
    }
}
