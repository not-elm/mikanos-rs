#![feature(pointer_byte_offsets)]
#![no_main]
#![no_std]
#![feature(custom_test_frameworks)]
#![test_runner(test_runner::my_runner)]
#![reexport_test_harness_main = "test_main"]


use core::panic::PanicInfo;

use common_lib::frame_buffer::FrameBufferConfig;
use kernel_lib::gop::console::init_console;
use kernel_lib::println;

#[cfg(test)]
mod test_runner;

#[no_mangle]
pub extern "sysv64" fn kernel_main(frame_buffer_config: FrameBufferConfig) -> () {
    init_console(frame_buffer_config);
    println!("hello!");
    
    #[cfg(test)]
    test_main();
    
    common_lib::assembly::hlt_forever();
}

/// この関数はパニック時に呼ばれる
#[panic_handler]
#[cfg(not(test))]
fn panic(info: &PanicInfo) -> ! {
    println!("{:?}", info);
    common_lib::assembly::hlt_forever();
}


#[panic_handler]
#[cfg(test)]
fn panic(info: &PanicInfo) -> ! {
    println!("[test failed!]");
    println!("{:?}", info);
    common_lib::assembly::hlt_forever();
}

