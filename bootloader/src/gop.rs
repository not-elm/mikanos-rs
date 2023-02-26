use uefi::proto::console::gop::GraphicsOutput;
use uefi::table::{Boot, SystemTable};
use uefi::table::boot::ScopedProtocol;

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

pub unsafe fn write_all_pixels_with_same(gop: &mut ScopedProtocol<GraphicsOutput>, pixel_value: u8) {
    let mut frame_buffer = gop.frame_buffer();
    let buffer_size = frame_buffer.size();

    for i in 0..buffer_size {
        frame_buffer.write_byte(i, pixel_value);
    }
}