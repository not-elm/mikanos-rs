use core::arch::asm;

/// プロセッサを停止させます
pub fn hlt(){
   unsafe { asm!("hlt");}
}

pub fn hlt_forever(){
    loop {
        hlt();
    }
}