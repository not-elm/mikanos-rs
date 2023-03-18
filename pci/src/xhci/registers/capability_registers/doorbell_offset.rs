use macros::VolatileBits;

/// FIXME 下位2ビットが予約領域のため、ビットマスクするように修正する必要あり
#[derive(VolatileBits)]
#[volatile_type(u32)]
pub struct DoorbellOffset(usize);
