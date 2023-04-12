use core::arch::asm;

pub fn read_cr() -> u16 {
    let segment: u16;
    unsafe {
        asm!("mov {0:x}, cr", out(reg) segment, options(nomem,  nostack, preserves_flags));
    }
    segment
}
