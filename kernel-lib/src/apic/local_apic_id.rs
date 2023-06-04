use volatile_bits::volatile_bits;

use crate::apic::LocalApicRegistersAddr;

#[volatile_bits(
type = u8,
add = 0x20
)]
pub struct LocalApicId(LocalApicRegistersAddr);


impl LocalApicId {
    pub fn new(addr: LocalApicRegistersAddr) -> Self {
        Self::from(addr)
    }
}
