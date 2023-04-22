use common_lib::math::rectangle::Rectangle;
use common_lib::math::vector::Vector2D;
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
        _prev_cursor: Vector2D<usize>,
        _current_cursor: Vector2D<usize>,
        _button: Option<MouseButton>,
    ) -> Result<(), ()> {
        // if is_drawable_cursor_pos(self.frame_buffer_rect, prev_cursor) {
        //     erase_cursor(prev_cursor).map_err(|_| ())?;
        // }

        // if is_drawable_cursor_pos(self.frame_buffer_rect, current_cursor) {
        //
        // }

        // let color = button
        //     .map(|b| match b {
        //         MouseButton::Button1 => PixelColor::yellow(),
        //         MouseButton::Button2 => PixelColor::new(0x13, 0xA9, 0xDB),
        //         MouseButton::Button3 => PixelColor::new(0x35, 0xFA, 0x66),
        //         _ => PixelColor::white(),
        //     })
        //     .unwrap_or(PixelColor::white());

        let layers = LAYERS.layers_mut();
        let mut layers = layers.borrow_mut();
        let layer = layers.layer_mut_at(0);


        let mouse_window = layer
            .window_mut("mouse")
            .map_err(|_| ())?;


        layer.draw_all().unwrap();

        Ok(())
    }
}
