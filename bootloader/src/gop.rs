use uefi::proto::console::gop::GraphicsOutput;
use uefi::table::boot::ScopedProtocol;
use uefi::table::{Boot, SystemTable};
use uefi_services::println;

use common_lib::frame_buffer::{FrameBufferConfig, PixelFormat};

pub fn open_gop(system_table: &SystemTable<Boot>) -> uefi::Result<ScopedProtocol<GraphicsOutput>> {
    let gop = {
        system_table
            .boot_services()
            .get_handle_for_protocol::<GraphicsOutput>()
    }?;
    system_table
        .boot_services()
        .open_protocol_exclusive::<GraphicsOutput>(gop)
}

pub fn obtain_frame_buffer_config(gop: &mut ScopedProtocol<GraphicsOutput>) -> FrameBufferConfig {
    let (frame_buffer_base, frame_buffer_size) = obtain_frame_buffer_base_addr_and_size(gop);
    let mode = gop.current_mode_info();
    let (horizontal_resolution, vertical_resolution) = mode.resolution();
    let pixel_format = match mode.pixel_format() {
        uefi::proto::console::gop::PixelFormat::Rgb => PixelFormat::Rgb,
        uefi::proto::console::gop::PixelFormat::Bgr => PixelFormat::Bgr,
        _ => PixelFormat::Bgr,
    };

    FrameBufferConfig::new(
        frame_buffer_base,
        frame_buffer_size,
        mode.stride(),
        horizontal_resolution,
        vertical_resolution,
        pixel_format,
    )
}

fn obtain_frame_buffer_base_addr_and_size(
    gop: &mut ScopedProtocol<GraphicsOutput>,
) -> (u64, usize) {
    let mut frame_buffer = gop.frame_buffer();

    let buffer_size = frame_buffer.size();

    let base_addr = frame_buffer.as_mut_ptr().addr() as u64;

    (base_addr, buffer_size)
}

#[allow(dead_code)]
pub unsafe fn write_all_pixels_with_same(
    gop: &mut ScopedProtocol<GraphicsOutput>,
    pixel_value: u8,
) {
    let mut frame_buffer = gop.frame_buffer();
    let buffer_size = frame_buffer.size();

    for i in 0..buffer_size {
        frame_buffer.write_byte(i, pixel_value);
    }
}
