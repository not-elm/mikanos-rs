#![feature(pointer_byte_offsets)]
#![no_main]
#![no_std]
#![feature(custom_test_frameworks)]
#![test_runner(my_runner)]
#![reexport_test_harness_main = "test_main"]


fn my_runner(tests: &[&i32]) {
    println!("test ");
    for t in tests {
        println!("test ");
        assert_eq!(0, **t);
        println!("success!");
    }
}

#[test_case]
const WILL_PASS: i32 = 0;

#[test_case]
const WILL_FAIL: i32 = 4;

use core::panic::PanicInfo;
use common_lib::frame_buffer::FrameBufferConfig;
use kernel_lib::gop::console::init_console;
use kernel_lib::println;

#[no_mangle]
pub extern "sysv64" fn kernel_main(frame_buffer_config: FrameBufferConfig) -> () {
    init_console(frame_buffer_config);
    println!("hello!");
    #[cfg(test)]
    println!("hello! test");

    #[cfg(test)]
    test_main();
   
    common_lib::assembly::hlt_forever();
}

/// この関数はパニック時に呼ばれる
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{:?}", info);
    common_lib::assembly::hlt_forever();
}
