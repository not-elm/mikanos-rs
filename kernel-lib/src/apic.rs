use volatile_bits::volatile_address;

use crate::apic::current_count::CurrentCount;
use crate::apic::device_config::DivideConfig;
use crate::apic::initial_count::InitialCount;
use crate::apic::local_apic_id::LocalApicId;
use crate::apic::lvt_timer::{LvtTimer, LvtTimerAddr};

use self::end_of_interrupt::EndOfInterrupt;

pub mod current_count;
pub mod device_config;
pub mod end_of_interrupt;
pub mod initial_count;
pub mod local_apic_id;
pub mod lvt_timer;

#[volatile_address]
pub struct LocalApicRegistersAddr(u64);


pub struct LocalApicRegisters {
    local_apic_id: LocalApicId,
    end_of_interrupt: EndOfInterrupt,
    lvt_timer: LvtTimer,
    initial_count: InitialCount,
    current_count: CurrentCount,
    divide_config: DivideConfig,
}


impl Default for LocalApicRegistersAddr {
    fn default() -> Self {
        LocalApicRegistersAddr::from(0xFEE00000)
    }
}


impl LocalApicRegisters {
    pub fn new(local_apic_addr: LocalApicRegistersAddr) -> Self {
        Self {
            local_apic_id: LocalApicId::new(local_apic_addr),
            end_of_interrupt: EndOfInterrupt::new(local_apic_addr),
            lvt_timer: LvtTimer::from(LvtTimerAddr::default()),
            initial_count: InitialCount::new(local_apic_addr),
            current_count: CurrentCount::new(local_apic_addr),
            divide_config: DivideConfig::new(local_apic_addr),
        }
    }

    pub fn local_apic_id(&self) -> &LocalApicId {
        &self.local_apic_id
    }


    pub fn end_of_interrupt(&self) -> &EndOfInterrupt {
        &self.end_of_interrupt
    }


    pub fn lvt_timer(&self) -> &LvtTimer {
        &self.lvt_timer
    }


    pub fn initial_count(&self) -> &InitialCount {
        &self.initial_count
    }


    pub fn current_count(&self) -> &CurrentCount {
        &self.current_count
    }


    pub fn divide_config(&self) -> &DivideConfig {
        &self.divide_config
    }
}


impl Default for LocalApicRegisters {
    fn default() -> Self {
        Self::new(LocalApicRegistersAddr::default())
    }
}
