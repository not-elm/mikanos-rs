#![no_main]
#![no_std]
#![allow(stable_features)]


use core::arch::asm;

/// プロセッサを停止させます
pub unsafe fn hlt(){
    asm!("hlt")
}