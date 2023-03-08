#![feature(pointer_byte_offsets)]
#![no_main]
#![no_std]

use common_lib::frame_buffer::FrameBufferConfig;
use common_lib::vector::Vector2D;
use core::panic::PanicInfo;
use kernel_lib::error::KernelResult;

use kernel_lib::gop::console::{draw_cursor, fill_rect_using_global, init_console};
use kernel_lib::gop::pixel::pixel_color::PixelColor;
use kernel_lib::println;

#[no_mangle]
pub extern "sysv64" fn kernel_main(frame_buffer_config: FrameBufferConfig) -> () {
    init_console(frame_buffer_config);

    fill_background(PixelColor::new(0x3E, 0x3E, 0x3E), &frame_buffer_config).unwrap();
    fill_bottom_bar(PixelColor::new(0x00, 0x00, 0xFF), &frame_buffer_config).unwrap();

    draw_cursor().unwrap();

    common_lib::assembly::hlt_forever();
}

fn fill_background(color: PixelColor, config: &FrameBufferConfig) -> KernelResult {
    fill_rect_using_global(
        Vector2D::new(0, 0),
        Vector2D::new(config.horizontal_resolution, config.vertical_resolution),
        color,
    )
}

fn fill_bottom_bar(color: PixelColor, config: &FrameBufferConfig) -> KernelResult {
    let v = config.vertical_resolution;
    let h = config.horizontal_resolution;
    fill_rect_using_global(Vector2D::new(0, v - 50), Vector2D::new(h, v), color)?;
    fill_rect_using_global(
        Vector2D::new(0, v - 50),
        Vector2D::new(50, v),
        PixelColor::new(0x33, 0x33, 0xAA),
    )
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
