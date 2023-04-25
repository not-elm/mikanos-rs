use common_lib::frame_buffer::FrameBufferConfig;

use crate::gop::pixel::calc_pixel_pos_from_vec2d;
use crate::gop::pixel::pixel_frame::PixelFrame;
use crate::{error::KernelResult, gop::pixel::pixel_color::PixelColor};

#[warn(drop_bounds)]
pub trait PixelWritable {
    /// # Safety
    /// Should be pass the correct frame buffer address and
    /// the pixel position must be with in the frame buffer area
    unsafe fn write(&mut self, x: usize, y: usize, color: &PixelColor) -> KernelResult;
}


pub trait PixelFlushable {
    ///# Safety
    /// Should be pass the correct frame buffer address and
    /// the pixel position must be with in the frame buffer area
    ///
    /// 描画域を表す構造体を元に、ピクセルを描画します。
    unsafe fn flush(&mut self, pixel_frame: PixelFrame) -> KernelResult;
}


pub(crate) unsafe fn flush_frame_buff(
    pixel_frame: PixelFrame,
    frame_buffer_config: &FrameBufferConfig,
) -> KernelResult {
    for row in pixel_frame.into_iter() {
        let frame_buff = core::slice::from_raw_parts_mut(
            frame_buffer_config.frame_buffer_base_ptr(),
            frame_buffer_config.frame_buffer_size,
        );
        let origin = calc_pixel_pos_from_vec2d(frame_buffer_config, row.origin_pos())?;
        let end = origin + row.pixels_len_per_row() - 1;

        let buff = row.pixels_buff();
        frame_buff[origin..=end].copy_from_slice(buff)
    }
    Ok(())
}
