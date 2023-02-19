#![no_main]
#![no_std]


use core::arch::asm;
use core::panic::PanicInfo;

#[no_mangle]
pub extern "sysv64" fn _start() -> () {
    unsafe {
        loop {
            asm!("hlt");
        }
    }
}

/// この関数はパニック時に呼ばれる
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}