#![feature(pointer_byte_offsets)]
#![no_main]
#![no_std]

use common_lib::frame_buffer::FrameBufferConfig;
use core::panic::PanicInfo;

use kernel_lib::gop::console::init_console;
use kernel_lib::println;

#[no_mangle]
pub extern "sysv64" fn kernel_main(frame_buffer_config: FrameBufferConfig) -> () {
    init_console(frame_buffer_config);
    for i in 0..26 {
        println!("Hello Rust World! {}", i);
    }

    common_lib::assembly::hlt_forever();
}

/// この関数はパニック時に呼ばれる
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("panic! {}", info);
    common_lib::assembly::hlt_forever();
}

#[allow(dead_code)]
unsafe fn fill_display_with_white(frame_buffer_base_addr: u64, frame_buffer_size: usize) {
    let frame_buffer = frame_buffer_base_addr as *mut u8;
    let frame_buffer = core::slice::from_raw_parts_mut(frame_buffer, frame_buffer_size);
    for i in 0..frame_buffer_size {
        frame_buffer[i] = 0xFF;
    }
}
