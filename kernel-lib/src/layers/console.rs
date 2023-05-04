use alloc::boxed::Box;

use common_lib::frame_buffer::FrameBufferConfig;
use common_lib::impl_transformable2D;
use common_lib::math::rectangle::Rectangle;
use common_lib::transform::transform2d::Transform2D;

use crate::error::KernelResult;
use crate::gop::char::ascii_char_writer::AscIICharWriter;
use crate::gop::console::console_builder::ConsoleBuilder;
use crate::gop::console::console_writer::ConsoleWriter;
use crate::gop::shadow_frame_buffer::ShadowFrameBuffer;
use crate::layers::console_colors::ConsoleColors;
use crate::layers::layer::Layer;
use crate::layers::layer_updatable::LayerUpdatable;
use crate::layers::{copy_frame_buff_in_area, frame_buffer_layer_transform};

pub struct ConsoleLayer {
    console_writer: ConsoleWriter<AscIICharWriter>,
    transform: Transform2D,
    shadow_buff: ShadowFrameBuffer,
}


impl ConsoleLayer {
    pub fn new(config: FrameBufferConfig, colors: ConsoleColors) -> Self {
        let transform = frame_buffer_layer_transform(config);

        let shadow_buff = ShadowFrameBuffer::new(config);

        Self {
            console_writer: ConsoleBuilder::new()
                .color(*colors.foreground())
                .build(config),
            transform,
            shadow_buff,
        }
    }


    pub fn into_enum(self) -> Layer {
        Layer::Console(Box::new(self))
    }


    pub fn write_str(&mut self, str: &str) -> KernelResult {
        self.console_writer
            .write_str(self.shadow_buff.raw_mut(), str)?;
        let frame_buff = unsafe {
            core::slice::from_raw_parts_mut(
                self.shadow_buff
                    .config_ref()
                    .frame_buffer_base_ptr(),
                self.shadow_buff
                    .config_ref()
                    .frame_buffer_size,
            )
        };

        self.console_writer
            .write_str(frame_buff, str)
    }
}


impl LayerUpdatable for ConsoleLayer {
    fn update_shadow_buffer(
        &mut self,
        shadow_buff: &mut ShadowFrameBuffer,
        draw_area: &Rectangle<usize>,
    ) -> KernelResult {
        return Ok(());
        copy_frame_buff_in_area(
            self.shadow_buff.raw_ref(),
            shadow_buff.raw_mut(),
            self.shadow_buff.config_ref(),
            draw_area,
        )
    }
}


impl_transformable2D!(ConsoleLayer);
