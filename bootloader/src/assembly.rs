use core::arch::asm;

/// CPUを停止させます
/// MikanOSの書籍によると、完全に停止するわけではなく、
/// 割り込みが発生するとCPUの動作が再開するようです。
pub fn hlt() {
    unsafe { asm!("hlt"); }
}

/// CPUを省電力にしつつ、永続ループを発生させます。
/// hltについては[hlt]を参照してください。
pub fn hlt_forever() -> ! {
    loop {
        hlt();
    }
}