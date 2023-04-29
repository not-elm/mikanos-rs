use crate::apic::LocalApicRegistersAddr;
use macros::VolatileBits;

#[derive(VolatileBits)]
#[add_addr_bytes(0x380)]
pub struct InitialCount(usize);


impl InitialCount {
    pub fn new(local_apic_addr: LocalApicRegistersAddr) -> Self {
        Self::new_uncheck(local_apic_addr.addr())
    }
}
