use kernel_lib::println;
use macros::Volatile;

use crate::error::{PciError, PciResult};

#[derive(Debug, Clone, Volatile)]
#[volatile_type(u32, right_shift = 2)]
pub struct DoorbellOffset(usize);

impl DoorbellOffset {
    pub(crate) fn new_with_check(addr: usize, cap_length: u8) -> PciResult<Self> {
        let db_offset =
            Self::new_non_zero(addr).ok_or(PciError::ZeroRegister("doorbell_offset"))?;
        println!("db_off {:X}", db_offset.read_volatile());
        println!("cap_length {:X}", cap_length);
        if db_offset.read_volatile() < cap_length as u32 {
            Err(PciError::DoorbellOffsetInvalid(db_offset.read_volatile()))
        } else {
            Ok(db_offset)
        }
    }
}
