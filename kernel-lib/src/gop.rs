use core::cell::OnceCell;

use common_lib::frame_buffer::FrameBufferConfig;

pub mod char;
pub mod console;
pub mod font;
pub mod pixel;
pub mod shadow_frame_buffer;


static FRAME_BUFFER_CONFIG: GlobalFrameBufferConfig = GlobalFrameBufferConfig(OnceCell::new());

struct GlobalFrameBufferConfig(OnceCell<FrameBufferConfig>);


unsafe impl Sync for GlobalFrameBufferConfig {}


/// Initialize Frame Buffer Config State
pub fn init(config: FrameBufferConfig) {
    FRAME_BUFFER_CONFIG
        .0
        .set(config)
        .unwrap();
}


/// Get Frame Buffer Config
///
/// # Panics
///
/// Panic if not called [`init`]
#[inline(always)]
pub fn config() -> FrameBufferConfig {
    *FRAME_BUFFER_CONFIG
        .0
        .get()
        .unwrap()
}


#[cfg(test)]
pub fn test_init() {
    let _ = FRAME_BUFFER_CONFIG
        .0
        .set(FrameBufferConfig::mock());
}
