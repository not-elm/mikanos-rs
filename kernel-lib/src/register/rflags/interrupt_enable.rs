use volatile_bits::volatile_bits;

#[volatile_bits(
type = u8,
bits = 1,
offset = 8
)]
pub  struct InterruptEnableFlag(u64);
