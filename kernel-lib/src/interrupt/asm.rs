use core::arch::asm;

use crate::interrupt::idt_descriptor::IdtDescriptor;

/// 割り込みを有効化します。
#[inline(always)]
pub fn sti() {
    unsafe {
        asm!("sti", options(nomem, nostack));
    }
}



#[inline(always)]
pub fn sti_and_nop() {
    unsafe {
        asm!("sti; nop", options(nomem, nostack));
    }
}


/// 割り込みを有効化し、CPUを休止させます。
#[inline(always)]
pub fn sti_and_hlt() {
    unsafe {
        asm!("sti; hlt", options(nomem, nostack));
    }
}


/// 割り込みを無効化します。
#[inline(always)]
pub fn cli() {
    unsafe {
        asm!("cli", options(nomem, nostack));
    }
}


/// Interrupt Descriptor Tableをロードします。
#[inline(always)]
pub fn load_idt(idt_descriptor: &IdtDescriptor) {
    unsafe {
        asm!("lidt [{}]", in(reg) idt_descriptor, options(readonly, nostack, preserves_flags));
    }
}
