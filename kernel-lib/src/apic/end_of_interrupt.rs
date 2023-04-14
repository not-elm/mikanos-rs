use macros::VolatileBits;

use crate::apic::LocalApicRegistersAddr;

/// EOI
#[derive(VolatileBits)]
#[volatile_type(u64)]
#[add_addr_bytes(0xB0)]
#[volatile_type(u32)]
pub struct EndOfInterrupt(usize);

impl EndOfInterrupt {
    pub fn new(addr: LocalApicRegistersAddr) -> Self {
        Self::new_uncheck(addr.0)
    }


    /// 割り込み処理が終了したことを通知します。
    pub fn notify(&self) {
        self.write_volatile(0);
    }
}
