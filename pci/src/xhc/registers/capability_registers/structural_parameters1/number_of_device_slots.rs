use macros::VolatileBits;

use crate::xhc::registers::capability_registers::structural_parameters1::StructuralParameters1Offset;

#[derive(VolatileBits)]
#[volatile_type(u8)]
/// 扱えるデバイスの数
/// MaxSlots
pub struct NumberOfDeviceSlots(usize);

impl NumberOfDeviceSlots {
    pub fn new(offset: StructuralParameters1Offset) -> Self {
        Self::new_uncheck(offset.addr())
    }
}
