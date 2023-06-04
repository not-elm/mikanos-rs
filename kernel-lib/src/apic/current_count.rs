use volatile_bits::volatile_bits;

use crate::apic::LocalApicRegistersAddr;

#[volatile_bits(add = 0x0390, type = u32)]
pub struct CurrentCount(LocalApicRegistersAddr);


impl CurrentCount {
    pub fn new(local_apic_addr: LocalApicRegistersAddr) -> Self {
        Self::from(local_apic_addr)
    }
}
