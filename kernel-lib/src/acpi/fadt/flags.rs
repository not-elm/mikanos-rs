use crate::volatile_bits::volatile_bits;

#[volatile_bits(
type = u32,
add = 112
)]
#[derive(Debug, Copy, Clone)]
pub struct Flags(u64);
