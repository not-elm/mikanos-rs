use kernel_lib::println;
use macros::VolatileFlag;

use crate::error::{PciError, PciResult};
use crate::xhci::registers::operation_registers::usb_status_register::usb_status_register_offset::UsbStatusRegisterOffset;

#[derive(VolatileFlag)]
pub struct HostControllerHalted(usize);

impl HostControllerHalted {
    pub fn new(offset: UsbStatusRegisterOffset) -> Self {
        println!("{:b}", offset.offset());

        Self::new_uncheck(offset.offset())
    }

    pub fn new_expect_halted(offset: UsbStatusRegisterOffset) -> PciResult<Self> {
        HostControllerHalted::new_expect_to_be(true, offset.offset())
            .ok_or(PciError::HostControllerNotHalted)
    }
}
