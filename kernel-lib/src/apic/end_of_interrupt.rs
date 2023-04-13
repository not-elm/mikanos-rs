use macros::VolatileBits;

use crate::apic::LocalApicRegistersAddr;

/// EOI
#[derive(VolatileBits)]
#[volatile_type(u64)]
#[add_addr_bytes(0xB0)]
#[volatile_type(u8)]
pub struct EndOfInterrupt(usize);

impl EndOfInterrupt {
    pub fn new(addr: LocalApicRegistersAddr) -> Self {
        Self(addr.addr())
    }


    pub fn eoi(&self) {
        self.write_volatile(0);
    }
}
