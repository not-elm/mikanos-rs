use macros::VolatileBits;

use crate::apic::LocalApicRegistersAddr;

#[derive(VolatileBits)]
#[add_addr_bytes(0x0390)]
pub struct CurrentCount(usize);


impl CurrentCount {
    pub fn new(local_apic_addr: LocalApicRegistersAddr) -> Self {
        Self::new_uncheck(local_apic_addr.addr())
    }
}
