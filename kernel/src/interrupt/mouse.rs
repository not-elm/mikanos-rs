use common_lib::vector::Vector2D;
use pci::class_driver::mouse::mouse_subscribable::MouseSubscribable;
use pci::class_driver::mouse::MouseButton;

#[derive(Debug)]
pub struct MouseObserver {
    frame_buffer_size: Vector2D<usize>,
}


impl MouseSubscribable for MouseObserver {
    fn subscribe(&mut self, prev_cursor: Vector2D<usize>, current_cursor: Vector2D<usize>, button: Option<MouseButton>) -> Result<(), ()> {}
}

fn on_mouse_move(
    prev_cursor: Vector2D<usize>,
    current_cursor: Vector2D<usize>,
    button: Option<MouseButton>,
) -> Result<(), ()> {
    let color = button
        .map(|b| match b {
            MouseButton::Button1 => PixelColor::yellow(),
            MouseButton::Button2 => PixelColor::new(0x13, 0xA9, 0xDB),
            MouseButton::Button3 => PixelColor::new(0x35, 0xFA, 0x66),
            _ => PixelColor::white(),
        })
        .unwrap_or(PixelColor::white());
    
    erase_cursor(prev_cursor).map_err(|_| ())?;
    draw_cursor(current_cursor, color).map_err(|_| ())
}
