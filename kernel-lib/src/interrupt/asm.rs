use crate::interrupt::idt_descriptor::IdtDescriptor;
use core::arch::asm;

/// 割り込みを有効化します。
#[inline]
pub fn sti() {
    unsafe {
        asm!("sti", options(nomem, nostack));
    }
}


/// 割り込みを有効化し、CPUを休止させます。
#[inline]
pub fn sti_and_hlt() {
    unsafe {
        asm!("sti; hlt", options(nomem, nostack));
    }
}


/// 割り込みを無効化します。
#[inline]
pub fn cli() {
    unsafe {
        asm!("cli", options(nomem, nostack));
    }
}


/// Interrupt Descriptor Tableをロードします。
#[inline]
pub fn load_idt(idt_descriptor: &IdtDescriptor) {
    unsafe {
        asm!("lidt [{}]", in(reg) idt_descriptor, options(readonly, nostack, preserves_flags));
    }
}
