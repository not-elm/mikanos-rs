use volatile_bits::{volatile_address, volatile_bit_field, VolatileAddress};

use crate::apic::lvt_timer::interrupt_id_number::InterruptIdNumber;
use crate::apic::lvt_timer::mask::Mask;
use crate::apic::lvt_timer::timer_mode::TimerModeField;
use crate::apic::LocalApicRegistersAddr;

pub mod interrupt_id_number;
pub mod mask;
pub mod timer_mode;


#[volatile_bit_field(addr_ty = LvtTimerAddr)]
pub struct LvtTimer {
    interrupt_id_num: InterruptIdNumber,
    mask: Mask,
    timer_mode: TimerModeField,
}


#[volatile_address]
pub struct LvtTimerAddr(u64);


impl Default for LvtTimerAddr {
    fn default() -> Self {
        Self::from(LocalApicRegistersAddr::default().address() + 0x320)
    }
}
