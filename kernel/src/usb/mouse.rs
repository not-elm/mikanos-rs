use common_lib::math::rectangle::Rectangle;
use common_lib::math::vector::Vector2D;
use kernel_lib::error::KernelError;
use kernel_lib::gop::pixel::pixel_color::PixelColor;
use pci::class_driver::mouse::mouse_subscribable::MouseSubscribable;
use pci::class_driver::mouse::MouseButton;

use crate::layers::LAYERS;

#[derive(Debug, Clone)]
pub struct MouseSubscriber {
    frame_buffer_rect: Rectangle<usize>,
}


impl MouseSubscriber {
    pub fn new(frame_buffer_width: usize, frame_buffer_height: usize) -> Self {
        Self {
            frame_buffer_rect: Rectangle::new(
                Vector2D::new(0, 0),
                Vector2D::new(frame_buffer_width, frame_buffer_height),
            ),
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
        // if is_drawable_cursor_pos(self.frame_buffer_rect, prev_cursor) {
        //     erase_cursor(prev_cursor).map_err(|_| ())?;
        // }

        // if is_drawable_cursor_pos(self.frame_buffer_rect, current_cursor) {
        //
        // }

        let color = button
            .map(|b| match b {
                MouseButton::Button1 => PixelColor::yellow(),
                MouseButton::Button2 => PixelColor::new(0x13, 0xA9, 0xDB),
                MouseButton::Button3 => PixelColor::new(0x35, 0xFA, 0x66),
                _ => PixelColor::white(),
            })
            .unwrap_or(PixelColor::white());

        let mut mutex = LAYERS.mutex().lock();
        let mut layer = mutex.layer_mut_at(0);


        layer.window_mut_at("mouse");
        // let layer = layers
        //     .layer_mut_at(0)
        //     .unwrap();
        {
            // layer
            //     .window_mut_at("mouse")
            //     .ok_or(())?
            //     .move_window(current_cursor);
        };


        // let layer = LAYERS.layer_at(0).unwrap();
        // layer.draw_all().unwrap();
        // draw_cursor(current_cursor, color).map_err(|_| ())?;
        Ok(())
    }
}
