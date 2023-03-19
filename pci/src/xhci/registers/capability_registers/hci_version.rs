use macros::VolatileBits;


#[derive(VolatileBits)]
#[volatile_type(u16)]
#[offset(16)]
pub struct HciVersion(usize);
