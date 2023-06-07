use crate::error::PciResult;

pub trait TryFromU32<T> {
    fn try_from_u32(raw_value: u32) -> PciResult<T>;
}
