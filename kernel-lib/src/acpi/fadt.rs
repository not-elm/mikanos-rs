use volatile_bits::{volatile_bit_field, VolatileBitsReadable};

use crate::acpi::fadt::flags::Flags;
use crate::acpi::fadt::pm_timer_block::PmTimerBlock;
use crate::io::asm::io_in32;

pub mod flags;
pub mod pm_timer_block;

#[volatile_bit_field(addr_ty = u64)]
#[derive(Debug, Clone)]
pub struct Fadt {
    pm_timer_block: PmTimerBlock,
    flags: Flags,
}


impl Fadt {
    pub fn wait_milli_for(&self, milli: u32) {
        const FREQ: u32 = 3579545;

        let start = self.read_count();
        let end = start + FREQ * milli / 1000;
        let end = if self.is_count_32_bits() {
            end
        } else {
            end & 0x00_FF_FF_FF
        };

        if end < start {
            while start <= self.read_count() {}
        }

        while self.read_count() < end {}
    }

    #[inline]
    fn read_count(&self) -> u32 {
        io_in32(
            self.pm_timer_block
                .read_volatile() as u16,
        )
    }

    #[inline]
    fn is_count_32_bits(&self) -> bool {
        let flag = (self.flags.read_volatile() >> 8) & 1;
        flag == 1
    }
}
