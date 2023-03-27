use crate::error::{InvalidRegisterReason, PciError, PciResult};
use crate::xhc::registers::internal::capability_registers::structural_parameters2::StructuralParameters2Offset;
use crate::VolatileAccessible;

pub trait StructuralParameters2Field<T, VolatileType, Addr>
where
    T: VolatileAccessible<VolatileType, Addr, StructuralParameters2Offset>,
{
    fn new(offset: StructuralParameters2Offset) -> T;
    fn new_check_flag_true(offset: StructuralParameters2Offset) -> PciResult<T>;
    fn new_check_flag_false(offset: StructuralParameters2Offset) -> PciResult<T>;
}

impl<T, VolatileType> StructuralParameters2Field<T, VolatileType, usize> for T
where
    T: VolatileAccessible<VolatileType, usize, StructuralParameters2Offset>,
{
    fn new(offset: StructuralParameters2Offset) -> T {
        T::new_uncheck(offset.offset())
    }

    fn new_check_flag_true(offset: StructuralParameters2Offset) -> PciResult<T> {
        let s = T::new(offset);
        if s.read_flag_volatile() {
            Ok(s)
        } else {
            Err(PciError::InvalidRegister(
                InvalidRegisterReason::IllegalBitFlag { expect: true },
            ))
        }
    }

    fn new_check_flag_false(offset: StructuralParameters2Offset) -> PciResult<T> {
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
