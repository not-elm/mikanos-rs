use crate::layers::MOUSE_LAYER_KEY;
use common_lib::frame_buffer::FrameBufferConfig;
use kernel_lib::layers::cursor::CursorLayer;
use kernel_lib::layers::layer_key::LayerKey;

pub(crate) fn mouse(config: FrameBufferConfig) -> LayerKey {
    LayerKey::new(
        MOUSE_LAYER_KEY,
        CursorLayer::new_use_default(config).into_enum(),
    )
}
