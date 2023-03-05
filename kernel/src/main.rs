#![feature(pointer_byte_offsets)]
#![no_main]
#![no_std]

use common_lib::frame_buffer::FrameBufferConfig;
use core::panic::PanicInfo;
use kernel_lib::gop::console_builder::ConsoleBuilder;

#[no_mangle]
pub extern "sysv64" fn kernel_main(frame_buffer_config: FrameBufferConfig) -> () {
    let mut console = ConsoleBuilder::new().build(frame_buffer_config);
    for _ in 0..230 {
        console.write_str("Hello !Mikan Rust World!\n").unwrap();
    }

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

/// この関数はパニック時に呼ばれる
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
