use core::arch::asm;

pub mod asm;
pub mod interrupt_descriptor;
pub mod interrupt_queue_waiter;

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