use alloc::string::ToString;
use core::fmt::Write;

use auto_delegate::Delegate;

use common_lib::math::size::Size;
use common_lib::math::vector::Vector2D;
use common_lib::transform::transform2d::Transform2D;

use crate::gop::config;
use crate::gop::pixel::pixel_color::PixelColor;
use crate::layers::layer::Layer;
use crate::layers::layer_key::LayerKey;
use crate::layers::multiple_layer::LayerFindable;
use crate::layers::shape::shape_drawer::ShapeDrawer;
use crate::layers::shape::ShapeLayer;
use crate::layers::text::command::{Command, CommandAction, CommandArgs, CommandResult};
use crate::layers::text::config;
use crate::layers::text_box::TextBoxLayer;
use crate::layers::window::WindowLayer;

const TEXT_BOX_LAYER_KEY: &str = "Terminal Text";

#[derive(Delegate)]
pub struct TerminalLayer {
    #[to(Transformable2D, LayerUpdatable, LayerFindable)]
    window: WindowLayer,
}


impl TerminalLayer {
    pub fn new(transform: Transform2D) -> TerminalLayer {
        let window = WindowLayer::new_dark_color("", transform)
            .then_add(text_background)
            .unwrap()
            .then_add(text)
            .unwrap();

        Self { window }
    }


    #[inline]
    pub fn into_enum(self) -> Layer {
        Layer::Terminal(self)
    }


    #[inline]
    pub(crate) fn window_mut(&mut self) -> &mut WindowLayer {
        &mut self.window
    }


    #[inline]
    pub fn delete_last(&mut self) {
        self.window
            .find_by_key_mut(TEXT_BOX_LAYER_KEY)
            .unwrap()
            .require_text_box()
            .unwrap()
            .delete_last()
    }

    #[inline]
    pub fn is_active(&self) -> bool {
        self.window.is_active()
    }
}


impl Write for TerminalLayer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.window
            .find_by_key_mut(TEXT_BOX_LAYER_KEY)
            .unwrap()
            .require_text_box()
            .unwrap()
            .write_str(s)
    }
}


fn text_background(size: Size) -> LayerKey {
    let drawer = ShapeDrawer::new(config(), PixelColor::black());
    ShapeLayer::new(drawer, Transform2D::new(Vector2D::zeros(), size))
        .into_enum()
        .into_layer_key("Terminal Text Background")
}


fn text(size: Size) -> LayerKey {
    let pos = Vector2D::zeros();
    let config = config::Builder::new()
        .set_scrollable()
        .prefix('>')
        .add_command(Command::new("echo", echo))
        .add_command(Command::new("clear", clear))
        .build();

    let text = TextBoxLayer::new_dark(Transform2D::new(pos, size), config);
    text.into_enum()
        .into_layer_key(TEXT_BOX_LAYER_KEY)
}


fn echo(args: CommandArgs) -> CommandResult {
    if args.is_empty() {
        return Err("Not Command Args".to_string());
    }

    Ok(CommandAction::Output(args[0].to_string()))
}


fn clear(_args: CommandArgs) -> CommandResult {
    Ok(CommandAction::Clear)
}


#[cfg(test)]
mod tests {
    use alloc::string::ToString;

    use crate::layers::terminal::echo;

    #[test]
    fn it_echo() {
        let output = echo(&["aaa"]);
        assert_eq!(output, Ok("aaa".to_string()));
    }
}
