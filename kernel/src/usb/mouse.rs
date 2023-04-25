use common_lib::math::rectangle::Rectangle;
use common_lib::math::size::Size;
use common_lib::math::vector::Vector2D;
use kernel_lib::error::KernelResult;
use kernel_lib::gop::pixel::pixel_color::PixelColor;
use kernel_lib::gop::pixel::writer::pixel_writable::{PixelFlushable, PixelWritable};
use kernel_lib::layers::layer::Layer;
use kernel_lib::layers::window::drawers::cursor::cursor_colors::CursorColors;
use kernel_lib::layers::window::drawers::cursor::mouse_cursor::MouseCursorDrawer;
use pci::class_driver::mouse::mouse_subscribable::MouseSubscribable;
use pci::class_driver::mouse::MouseButton;

use crate::layers::{LAYERS, MOUSE_LAYER_ID};

#[derive(Debug, Clone)]
pub struct MouseSubscriber {
    frame_buffer_rect: Rectangle<usize>,
}


impl MouseSubscriber {
    pub fn new(frame_buffer_width: usize, frame_buffer_height: usize) -> Self {
        Self {
            frame_buffer_rect: Rectangle::from_size(Size::new(
                frame_buffer_width,
                frame_buffer_height,
            )),
        }
    }
}


impl MouseSubscribable for MouseSubscriber {
    fn subscribe(
        &mut self,
        _prev_cursor: Vector2D<usize>,
        current_cursor: Vector2D<usize>,
        button: Option<MouseButton>,
    ) -> Result<(), ()> {
        let layers = LAYERS.layers_mut();
        let mut layers = layers.borrow_mut();
        let layer = layers.layer_mut_at(MOUSE_LAYER_ID);

        let window_rect = update_color(button, layer).map_err(|_| ())?;

        if layer
            .update_window_transform("mouse", |transform| transform.set_pos(current_cursor))
            .is_ok()
        {
            layers
                .draw_all_layers_in_area(0, &window_rect)
                .unwrap();
        }

        Ok(())
    }
}


fn update_color<Writer: PixelFlushable>(
    button: Option<MouseButton>,
    layer: &mut Layer<Writer>,
) -> KernelResult<Rectangle<usize>> {
    let cursor_color = button
        .map(|b| match b {
            MouseButton::Button1 => PixelColor::yellow(),
            MouseButton::Button2 => PixelColor::new(0x13, 0xA9, 0xDB),
            MouseButton::Button3 => PixelColor::new(0x35, 0xFA, 0x66),
            _ => PixelColor::white(),
        })
        .unwrap_or(PixelColor::white());

    let window = layer.window_mut("mouse")?;

    let drawer = window.drawer_down_cast_mut::<MouseCursorDrawer>()?;
    drawer.set_color(CursorColors::default().change_foreground(cursor_color));

    Ok(window.transform_ref().rect())
}
