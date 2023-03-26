use crate::error::{InvalidRegisterReason, PciError, PciResult};
use crate::xhc::registers::port_registers::port::PortRegisterAddr;
use crate::VolatileAccessible;

pub trait PortRegisterField<T, VolatileType, Addr>
where
    T: VolatileAccessible<VolatileType, Addr, PortRegisterAddr>,
{
    fn new(addr: PortRegisterAddr) -> T;
    fn new_check_flag_false(addr: PortRegisterAddr) -> PciResult<T>;
}

impl<T, VolatileType> PortRegisterField<T, VolatileType, usize> for T
where
    T: VolatileAccessible<VolatileType, usize, PortRegisterAddr>,
{
    fn new(addr: PortRegisterAddr) -> T {
        T::new_uncheck(addr.addr())
    }

    fn new_check_flag_false(addr: PortRegisterAddr) -> PciResult<T> {
        let s = T::new(addr);
        if s.read_flag_volatile() {
            Err(PciError::InvalidRegister(
                InvalidRegisterReason::IllegalBitFlag { expect: false },
            ))
        } else {
            Ok(s)
        }
    }
}
