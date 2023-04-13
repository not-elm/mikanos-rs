use core::arch::asm;

/// CodeSegmentを読み込みます
#[inline]
pub fn read_code_segment() -> u16 {
    let code_segment: u16;
    unsafe {
        asm!("mov {0:x}, cs", out(reg) code_segment, options(nomem, nostack, preserves_flags));
    }
    code_segment
}
