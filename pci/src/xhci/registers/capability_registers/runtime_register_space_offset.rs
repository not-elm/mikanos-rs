use macros::Volatile;

#[derive(Debug, Clone, Volatile)]
#[volatile_type(u32, right_shift = 5)]
pub struct RuntimeRegisterSpaceOffset(usize);
