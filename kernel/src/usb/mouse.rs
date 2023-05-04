use common_lib::math::size::Size;
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
pub struct MouseSubscriber {}


impl MouseSubscriber {
    pub fn new() -> Self {
        Self {}
    }
}


impl MouseSubscribable for MouseSubscriber {
    fn subscribe(
        &mut self,
        _prev_cursor: Vector2D<usize>,
        current_cursor: Vector2D<usize>,
        button: Option<MouseButton>,
    ) -> Result<(), ()> {
        let mut timer = OneShotLocalApicTimer::new();
        timer.start(LocalApicTimerDivide::By1);

        let layers = LAYERS.layers_mut();

        layers
            .borrow_mut()
            .update_layer(MOUSE_LAYER_KEY, |layer| {
                let color: PixelColor = cursor_color(button);
                if let Ok(cursor) = layer.require_cursor() {
                    cursor.move_to(current_cursor);

                    cursor.set_color(CursorColors::default().change_foreground(color))
                }
            })
            .map_err(|_| ())?;


        println!("Done Xhc All Events Time = {}", timer.elapsed());
        timer.stop();

        Ok(())
    }
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
