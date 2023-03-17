use macros::Volatile;

#[derive(Debug, Clone, Volatile)]
#[volatile_type(u32, right_shift = 2)]
pub struct DoorbellOffset(usize);
