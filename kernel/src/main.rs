#![feature(pointer_byte_offsets)]
#![no_main]
#![no_std]

use core::panic::PanicInfo;

use common_lib::frame_buffer::FrameBufferConfig;
use kernel_lib::frame::pixel::color::Color;
use kernel_lib::frame::pixel::writer::{write_pixel, RgbPixelWriter};

#[no_mangle]
pub extern "sysv64" fn kernel_main(frame_buffer_config: FrameBufferConfig) -> () {
    // let mut w = RgbPixelWriter {};
    // let color = Color::new(0xFF, 00, 00);
    // for x in 0..200 {
    //     for y in 0..100 {
    //         let config = FrameBufferConfig::new(30, frame_buffer_base_addr, frame_buffer_size);
    //         unsafe { write_pixel(&mut w, &config, x, y, &color) };
    //     }
    // }
    unsafe {
        fill_display_with_white(
            frame_buffer_config.frame_buffer_base,
            frame_buffer_config.frame_buffer_size,
        )
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
