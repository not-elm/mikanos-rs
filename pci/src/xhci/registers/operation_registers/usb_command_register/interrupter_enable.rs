use macros::VolatileBits;

#[derive(VolatileBits)]
pub struct InterrupterEnable(usize);
