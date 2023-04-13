use macros::Address;

use crate::apic::local_apic_id::LocalApicId;

use self::end_of_interrupt::EndOfInterrupt;

pub mod end_of_interrupt;
pub mod local_apic_id;

#[derive(Address, Clone, Copy)]
pub struct LocalApicRegistersAddr(usize);


pub struct LocalApicRegisters {
    local_apic_id: LocalApicId,
    end_of_interrupt: EndOfInterrupt,
}


impl LocalApicRegisters {
    pub fn new(addr: LocalApicRegistersAddr) -> Self {
        Self {
            local_apic_id: LocalApicId::new(addr),
            end_of_interrupt: EndOfInterrupt::new(addr),
        }
    }

    pub fn local_apic_id(&self) -> &LocalApicId {
        &self.local_apic_id
    }

    pub fn end_of_interrupt(&self) -> &EndOfInterrupt {
        &self.end_of_interrupt
    }
}


impl Default for LocalApicRegisters {
    fn default() -> Self {
        Self::new(LocalApicRegistersAddr::new(0xFEE00000))
    }
}