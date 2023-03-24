use crate::error::{InvalidRegisterReason, PciError, PciResult};
use crate::xhci::registers::capability_registers::structural_parameters1::StructuralParameters1Offset;
use crate::VolatileAccessible;

pub trait StructuralParameters1Field<T, VolatileType, Addr>
where
    T: VolatileAccessible<VolatileType, Addr, StructuralParameters1Offset>,
{
    fn new(offset: StructuralParameters1Offset) -> T;
    fn new_check_flag_true(offset: StructuralParameters1Offset) -> PciResult<T>;
    fn new_check_flag_false(offset: StructuralParameters1Offset) -> PciResult<T>;
}

impl<T, VolatileType> StructuralParameters1Field<T, VolatileType, usize> for T
where
    T: VolatileAccessible<VolatileType, usize, StructuralParameters1Offset>,
{
    fn new(offset: StructuralParameters1Offset) -> T {
        T::new_uncheck(offset.addr())
    }

    fn new_check_flag_true(offset: StructuralParameters1Offset) -> PciResult<T> {
        let s = T::new(offset);
        if s.read_flag_volatile() {
            Ok(s)
        } else {
            Err(PciError::InvalidRegister(
                InvalidRegisterReason::IllegalBitFlag { expect: true },
            ))
        }
    }

    fn new_check_flag_false(offset: StructuralParameters1Offset) -> PciResult<T> {
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
