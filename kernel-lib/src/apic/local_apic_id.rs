use macros::VolatileBits;

use crate::apic::LocalApicRegistersAddr;

#[derive(VolatileBits)]
#[add_addr_bytes(0x20)]
#[volatile_type(u8)]
pub struct LocalApicId(usize);


impl LocalApicId {
    pub fn new(addr: LocalApicRegistersAddr) -> Self {
        Self::new_uncheck(addr.addr())
    }
}
