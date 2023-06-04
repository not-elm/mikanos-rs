use volatile_bits::{volatile_bits, VolatileBitsWritable};

use crate::apic::LocalApicRegistersAddr;

#[volatile_bits(
bits = 4,
type = u32,
add = 0x3E0,
)]
pub struct DivideConfig(LocalApicRegistersAddr);


impl DivideConfig {
    pub fn new(local_apic_addr: LocalApicRegistersAddr) -> Self {
        Self(local_apic_addr)
    }

    pub fn update_divide(&self, divide: LocalApicTimerDivide) {
        self.write_volatile(divide as u32)
            .unwrap();
    }
}


#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
#[repr(u8)]
pub enum LocalApicTimerDivide {
    By2 = 0b0000,
    By4 = 0b0001,
    By8 = 0b0010,
    By16 = 0b0011,
    By32 = 0b1000,
    By64 = 0b1001,
    By128 = 0b1010,
    By1 = 0b1011,
}
