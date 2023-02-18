#![no_main]
#![no_std]


use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn efi_main() -> ! {
    common::assembly::hlt_forever()
}

/// この関数はパニック時に呼ばれる
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}