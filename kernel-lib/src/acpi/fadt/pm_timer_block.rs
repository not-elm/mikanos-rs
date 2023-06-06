use volatile_bits::volatile_bits;

#[volatile_bits(
type = u32,
add = 76
)]
#[derive(Debug, Copy, Clone)]
pub struct PmTimerBlock(u64);
