use core::arch::asm;

#[inline]
pub fn set_cr3(cr3: u64) {
    unsafe {
        asm!("mov cr3, {}", in(reg) cr3, options(nostack, preserves_flags));
    }
}


#[inline]
pub fn read_cr3() -> u64 {
    let cr3: u64;
    unsafe {
        asm!("mov {}, cr3", out(reg) cr3, options(readonly, nostack, preserves_flags));
    }

    cr3
}
