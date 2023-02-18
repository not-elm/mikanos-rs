use core::arch::asm;

/// プロセッサを停止させます
pub unsafe fn hlt(){
    asm!("hlt")
}