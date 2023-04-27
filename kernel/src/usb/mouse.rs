use common_lib::math::rectangle::Rectangle;
use common_lib::math::size::Size;
use common_lib::math::vector::Vector2D;
use common_lib::transform::Transform2D;
use kernel_lib::error::KernelResult;
use kernel_lib::gop::console::DISPLAY_BACKGROUND_COLOR;
use kernel_lib::gop::pixel::pixel_color::PixelColor;
use kernel_lib::gop::pixel::writer::pixel_writable::{PixelFlushable, PixelWritable};
use kernel_lib::layers::layer::Layer;
use kernel_lib::layers::window::drawers::cursor::cursor_buffer::{CURSOR_HEIGHT, CURSOR_WIDTH};
use kernel_lib::layers::window::drawers::cursor::cursor_colors::CursorColors;
use kernel_lib::layers::window::drawers::cursor::mouse_cursor::MouseCursorDrawer;
use pci::class_driver::mouse::mouse_subscribable::MouseSubscribable;
use pci::class_driver::mouse::MouseButton;

use crate::layers::{BACKGROUND_LAYER_ID, LAYERS, MOUSE_LAYER_ID};

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
        prev_cursor: Vector2D<usize>,
        current_cursor: Vector2D<usize>,
        button: Option<MouseButton>,
    ) -> Result<(), ()> {
        let layers = LAYERS.layers_mut();
        let mut layers = layers.borrow_mut();

        let prev_cursor = Vector2D::new(prev_cursor.x());
        
        {
            let layer = layers.layer_mut_at(MOUSE_LAYER_ID);


            let rect = Rectangle::from_pos_and_size(prev_cursor, Size::new(CURSOR_WIDTH, CURSOR_HEIGHT));
            let window = layer.window_mut("mouse").unwrap();
            let rect = window.transform_ref().rect();
            let drawer = window.drawer_down_cast_mut::<MouseCursorDrawer>().unwrap();

            drawer.set_color(CursorColors::default().change_foreground(DISPLAY_BACKGROUND_COLOR).change_border(DISPLAY_BACKGROUND_COLOR));
            layers
                .draw_all_layers_in_area(0, &rect)
                .map_err(|_| ())?
        }

        let layer = layers.layer_mut_at(MOUSE_LAYER_ID);


        update_color(button.clone(), layer).map_err(|_| ())?;


        if let Ok(transform) = layer
            .update_window_transform("mouse", |transform| transform.set_pos(current_cursor)) {
            layers
                .draw_all_layers_in_area(0, &transform.rect())
                .map_err(|_| ())?
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
