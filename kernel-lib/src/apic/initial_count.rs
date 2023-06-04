use volatile_bits::volatile_bits;

use crate::apic::LocalApicRegistersAddr;

#[volatile_bits(add = 0x380, type = u32)]
pub struct InitialCount(LocalApicRegistersAddr);


impl InitialCount {
    pub fn new(local_apic_addr: LocalApicRegistersAddr) -> Self {
        Self::from(local_apic_addr)
    }
}
