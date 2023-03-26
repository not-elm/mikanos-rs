use crate::error::{InvalidRegisterReason, PciError, PciResult};
use crate::xhc::registers::operational_registers::command_ring_control_register::CommandRingControlRegisterOffset;
use crate::VolatileAccessible;

pub trait CrcrField<T, VolatileType, Addr>
where
    T: VolatileAccessible<VolatileType, Addr, CommandRingControlRegisterOffset>,
{
    fn new(offset: CommandRingControlRegisterOffset) -> T;
    fn new_check_flag_false(offset: CommandRingControlRegisterOffset) -> PciResult<T>;
}

impl<T, VolatileType> CrcrField<T, VolatileType, usize> for T
where
    T: VolatileAccessible<VolatileType, usize, CommandRingControlRegisterOffset>,
{
    fn new(offset: CommandRingControlRegisterOffset) -> T {
        T::new_uncheck(offset.offset())
    }

    fn new_check_flag_false(offset: CommandRingControlRegisterOffset) -> PciResult<T> {
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
