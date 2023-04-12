use core::arch::asm;

#[inline]
pub fn load_idt() {
    unsafe {
        asm!("lidt [{}]", in(reg),  options(nomem, nostack));
    }
}


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
