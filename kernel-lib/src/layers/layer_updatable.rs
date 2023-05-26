use auto_delegate::delegate;

use common_lib::math::rectangle::Rectangle;

use crate::error::KernelResult;
use crate::gop::shadow_frame_buffer::ShadowFrameBuffer;

#[delegate]
pub trait LayerUpdatable {
    /// Update Back buffer corresponding to frame buffer
    fn update_back_buffer(
        &mut self,
        back_buff: &mut ShadowFrameBuffer,
        draw_area: &Rectangle<usize>,
    ) -> KernelResult;
}
