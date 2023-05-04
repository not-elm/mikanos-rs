use common_lib::math::rectangle::Rectangle;

use crate::error::KernelResult;
use crate::gop::shadow_frame_buffer::ShadowFrameBuffer;


pub trait LayerUpdatable {
    /// 指定された領域内で現在のウィンドウの状態を描画します。
    fn update_shadow_buffer(
        &mut self,
        shadow_frame_buff: &mut ShadowFrameBuffer,
        draw_area: &Rectangle<usize>,
    ) -> KernelResult;
}
