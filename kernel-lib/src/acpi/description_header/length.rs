use volatile_bits::volatile_bits;

#[volatile_bits(
type = u32,
add = 4
)]
#[derive(Debug, Copy, Clone)]
pub struct Length(u64);
