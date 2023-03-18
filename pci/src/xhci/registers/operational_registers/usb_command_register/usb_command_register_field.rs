use crate::error::{InvalidRegisterReason, PciError, PciResult};
use crate::xhci::registers::operational_registers::operation_registers_offset::OperationalRegistersOffset;
use crate::VolatileAccessible;

pub trait UsbCommandRegisterField<T, VolatileType, Addr>
where
    T: VolatileAccessible<VolatileType, Addr, OperationalRegistersOffset>,
{
    fn new(offset: OperationalRegistersOffset) -> T;
    fn new_check_flag_false(offset: OperationalRegistersOffset) -> PciResult<T>;
}

impl<T, VolatileType> UsbCommandRegisterField<T, VolatileType, usize> for T
where
    T: VolatileAccessible<VolatileType, usize, OperationalRegistersOffset>,
{
    fn new(offset: OperationalRegistersOffset) -> T {
        T::new_uncheck(offset.offset())
    }

    fn new_check_flag_false(offset: OperationalRegistersOffset) -> PciResult<T> {
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
