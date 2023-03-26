use macros::VolatileBits;

#[derive(VolatileBits)]
#[volatile_type(u32)]
pub struct CapabilityParameters1(usize);
