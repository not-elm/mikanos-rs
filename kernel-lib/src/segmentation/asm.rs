use core::arch::asm;

/// CS(コードセグメントレジスタ)を読み込みます
#[inline]
pub fn read_code_segment() -> u16 {
    let code_segment: u16;
    unsafe {
        asm!("mov {0:x}, cs", out(reg) code_segment, options(nomem, nostack, preserves_flags));
    }
    code_segment
}


#[inline]
pub fn read_stack_segment() -> u16 {
    let stack_segment: u16;
    unsafe {
        asm!("mov {0:x}, ss", out(reg) stack_segment, options(nomem, nostack, preserves_flags));
    }
    stack_segment
}
