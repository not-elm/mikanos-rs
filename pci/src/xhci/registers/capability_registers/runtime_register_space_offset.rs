use macros::VolatileBits;

/// FIXME 下位5ビットが予約領域のため、ビットマスクするように修正する必要あり
#[derive(VolatileBits)]
#[volatile_type(u32)]
pub struct RuntimeRegisterSpaceOffset(usize);
