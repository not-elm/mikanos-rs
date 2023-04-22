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
        current_cursor: Vector2D<usize>,
        _button: Option<MouseButton>,
    ) -> Result<(), ()> {
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


        layer
            .update_transform("mouse", |status| status.set_pos(current_cursor))
            .map_err(|_| ())?;

        layers
            .draw_all_layers_start_at(2)
            .map_err(|_| ())?;

        Ok(())
    }
}
