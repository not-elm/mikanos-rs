use alloc::string::String;

use common_lib::math::vector::Vector2D;
use common_lib::transform::transform2d::Transformable2D;
use kernel_lib::apic::device_config::LocalApicTimerDivide;
use kernel_lib::gop::pixel::pixel_color::PixelColor;
use kernel_lib::layers::cursor::cursor_colors::CursorColors;
use kernel_lib::timer::apic::local_apic_timer::OneShotLocalApicTimer;
use kernel_lib::timer::apic::ApicTimer;
use pci::class_driver::mouse::mouse_subscribable::MouseSubscribable;
use pci::class_driver::mouse::MouseButton;

use crate::layers::{LAYERS, MOUSE_LAYER_KEY};
use crate::println;

#[derive(Debug, Clone)]
pub struct MouseSubscriber;


impl MouseSubscriber {
    pub fn new() -> Self {
        Self {}
    }
}


impl MouseSubscribable for MouseSubscriber {
    fn subscribe(
        &mut self,
        prev_cursor: Vector2D<usize>,
        current_cursor: Vector2D<usize>,
        prev_button: Option<MouseButton>,
        button: Option<MouseButton>,
    ) -> Result<(), ()> {
        let mut timer = OneShotLocalApicTimer::new();
        timer.start(LocalApicTimerDivide::By1);

        {
            update_cursor_layer(current_cursor, button)?;
            update_draggable_layer(
                prev_cursor,
                current_cursor,
                prev_button,
                button,
                timer.elapsed() as usize,
            )?;
        }

        println!("Done Xhc All Events Time = {}", timer.elapsed());
        timer.stop();

        Ok(())
    }
}


fn update_draggable_layer(
    prev_cursor: Vector2D<usize>,
    current_cursor: Vector2D<usize>,
    prev_button: Option<MouseButton>,
    button: Option<MouseButton>,
    count: usize,
) -> Result<(), ()> {
    let prev_drag = prev_button.is_some_and(|b| matches!(b, MouseButton::Button1));
    let current_drag = button.is_some_and(|b| matches!(b, MouseButton::Button1));

    if prev_drag && current_drag {
        let relative = current_cursor.relative(prev_cursor);

        if let Some(window_key) =
            draggable_layer_key(&prev_cursor).or(draggable_layer_key(&current_cursor))
        {
            LAYERS
                .layers_mut()
                .lock()
                .borrow_mut()
                .update_layer(window_key.as_str(), |layer| {
                    layer.move_to_relative(relative);
                    layer
                        .require_window()
                        .unwrap()
                        .write_count(count);
                })
                .map_err(|_| ())?;
        }
    }
    Ok(())
}


fn draggable_layer_key(current_cursor: &Vector2D<usize>) -> Option<String> {
    LAYERS
        .layers_mut()
        .lock()
        .borrow()
        .find_window_layer_by_pos(current_cursor)
        .map(String::from)
}


fn update_cursor_layer(
    current_cursor: Vector2D<usize>,
    button: Option<MouseButton>,
) -> Result<(), ()> {
    let layers = LAYERS.layers_mut();
    let lock = layers.lock();

    let mut layer = lock.borrow_mut();
    layer
        .update_layer(MOUSE_LAYER_KEY, |layer| {
            let color: PixelColor = cursor_color(button);
            if let Ok(cursor) = layer.require_cursor() {
                cursor.move_to(current_cursor);

                cursor.set_color(CursorColors::default().change_foreground(color))
            }
        })
        .map_err(|_| ())
}


fn cursor_color(button: Option<MouseButton>) -> PixelColor {
    button
        .map(|b| match b {
            MouseButton::Button1 => PixelColor::yellow(),
            MouseButton::Button2 => PixelColor::new(0x13, 0xA9, 0xDB),
            MouseButton::Button3 => PixelColor::new(0x35, 0xFA, 0x66),
            _ => PixelColor::white(),
        })
        .unwrap_or(PixelColor::white())
}
