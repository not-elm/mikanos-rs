use crate::error::{InvalidRegisterReason, PciError, PciResult};
use crate::xhci::registers::runtime_registers::interrupter_register_set::InterrupterRegisterSetOffset;
use crate::VolatileAccessible;

pub trait InterrupterRegisterSetField<T, VolatileType, Addr>
where
    T: VolatileAccessible<VolatileType, Addr, InterrupterRegisterSetOffset>,
{
    fn new(offset: InterrupterRegisterSetOffset) -> T;
    fn new_check_flag_true(offset: InterrupterRegisterSetOffset) -> PciResult<T>;
    fn new_check_flag_false(offset: InterrupterRegisterSetOffset) -> PciResult<T>;
}

impl<T, VolatileType> InterrupterRegisterSetField<T, VolatileType, usize> for T
where
    T: VolatileAccessible<VolatileType, usize, InterrupterRegisterSetOffset>,
{
    fn new(offset: InterrupterRegisterSetOffset) -> T {
        T::new_uncheck(offset.offset())
    }

    fn new_check_flag_true(offset: InterrupterRegisterSetOffset) -> PciResult<T> {
        let s = T::new(offset);
        if s.read_flag_volatile() {
            Ok(s)
        } else {
            Err(PciError::InvalidRegister(
                InvalidRegisterReason::IllegalBitFlag { expect: true },
            ))
        }
    }

    fn new_check_flag_false(offset: InterrupterRegisterSetOffset) -> PciResult<T> {
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
