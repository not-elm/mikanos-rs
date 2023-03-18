use crate::xhci::registers::operational_registers::command_ring_control_register::CommandRingControlRegisterOffset;
use crate::VolatileAccessible;

pub trait CrcrField<T, VolatileType, Addr>
where
    T: VolatileAccessible<VolatileType, Addr>,
{
    fn new(offset: CommandRingControlRegisterOffset) -> T;
}

impl<T, VolatileType> CrcrField<T, VolatileType, usize> for T
where
    T: VolatileAccessible<VolatileType, usize>,
{
    fn new(offset: CommandRingControlRegisterOffset) -> T {
        T::new_uncheck(offset.offset())
    }
}
// pub fn new(offset: CommandRingControlRegisterOffset) -> PciResult<Self> {
//      let s = Self::new_uncheck(offset.offset());
//      if s.read_flag_volatile() {
//          Err(PciError::InvalidRegister(
//              InvalidRegisterReason::IllegalBitFlag { expect: false },
//          ))
//      } else {
//          Ok(s)
//      }
//  }
