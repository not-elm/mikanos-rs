use crate::error::{InvalidRegisterReason, PciError, PciResult};
use crate::VolatileAccessible;
use crate::xhci::registers::operational_registers::usb_status_register::usb_status_register_offset::UsbStatusRegisterOffset;

pub trait UsbStatusRegisterField<T, VolatileType, Addr>
where
    T: VolatileAccessible<VolatileType, Addr, UsbStatusRegisterOffset>,
{
    fn new(offset: UsbStatusRegisterOffset) -> Self;
    fn new_check_flag_true(offset: UsbStatusRegisterOffset) -> PciResult<T>;
    fn new_check_flag_false(offset: UsbStatusRegisterOffset) -> PciResult<T>;
}

impl<T, VolatileType> UsbStatusRegisterField<T, VolatileType, usize> for T
where
    T: VolatileAccessible<VolatileType, usize, UsbStatusRegisterOffset>,
{
    fn new(offset: UsbStatusRegisterOffset) -> T {
        T::new_uncheck(offset.offset())
    }

    fn new_check_flag_true(offset: UsbStatusRegisterOffset) -> PciResult<T> {
        let s = T::new(offset);
        if s.read_flag_volatile() {
            Ok(s)
        } else {
            Err(PciError::InvalidRegister(
                InvalidRegisterReason::IllegalBitFlag { expect: true },
            ))
        }
    }

    fn new_check_flag_false(offset: UsbStatusRegisterOffset) -> PciResult<T> {
        let s = T::new(offset);
        if s.read_flag_volatile() {
            Err(PciError::InvalidRegister(
                InvalidRegisterReason::IllegalBitFlag { expect: false },
            ))
        } else {
            Ok(s)
        }
    }
}
