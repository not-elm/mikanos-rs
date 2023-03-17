use crate::error::{PciError, PciResult};
use macros::Volatile;

#[derive(Debug, Clone, Volatile)]
#[volatile_type(u32, right_shift = 5)]
pub struct RuntimeRegisterSpaceOffset(usize);
impl RuntimeRegisterSpaceOffset {
    pub(crate) fn new_with_check(addr: usize, db_off: u32) -> PciResult<Self> {
        let rts_offset = Self::new_non_zero(addr)
            .ok_or(PciError::ZeroRegister("runtime_register_space_offset"))?;

        if rts_offset.read_volatile() < db_off {
            Err(PciError::RuntimeOffsetInvalid(rts_offset.read_volatile()))
        } else {
            Ok(rts_offset)
        }
    }
}
