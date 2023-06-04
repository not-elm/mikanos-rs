use volatile_bits::{volatile_bits, VolatileBitsWritable};

use crate::apic::LocalApicRegistersAddr;

/// EOI
#[volatile_bits(
type = u32,
add = 0xB0
)]
pub struct EndOfInterrupt(LocalApicRegistersAddr);

impl EndOfInterrupt {
    pub fn new(addr: LocalApicRegistersAddr) -> Self {
        Self::from(addr)
    }


    /// 割り込み処理が終了したことを通知します。
    pub fn notify(&self) {
        self.write_volatile(0)
            .unwrap();
    }
}
