use core::arch::asm;

use common_lib::nums::FlagConvertible;

#[derive(Debug, Clone)]
pub struct RFlags {
    raw: u64,
}


impl RFlags {
    #[inline(always)]
    pub fn read() -> Self {
        Self {
            raw: read_rflags_raw()
        }
    }

    #[inline(always)]
    pub fn are_enable_interrupt(&self) -> bool {
        ((self.raw >> 9) & 1).is_true()
    }


    #[inline(always)]
    pub fn are_enable_disabled(&self) -> bool {
        !self.are_enable_interrupt()
    }
}


#[inline(always)]
fn read_rflags_raw() -> u64 {
    let raw: u64;

    unsafe {
        asm!("pushfq; pop {}", out(reg) raw, options(nomem, preserves_flags));
    }

    raw
}