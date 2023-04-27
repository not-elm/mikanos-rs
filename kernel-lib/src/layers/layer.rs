use alloc::boxed::Box;
use alloc::collections::BTreeMap;

use common_lib::math::rectangle::Rectangle;
use common_lib::transform::Transform2D;

use crate::error::{KernelError, KernelResult, LayerReason};
use crate::error::LayerReason::NotExistsKey;
use crate::gop::pixel::writer::pixel_writable::PixelFlushable;
use crate::layers::window::drawers::WindowDrawable;
use crate::layers::window::Window;
use crate::serial_println;

pub struct Layer<Writer> {
    layer_transform: Transform2D,
    pixel_writer: Writer,
    windows: BTreeMap<&'static str, Window<Box<dyn WindowDrawable>>>,
}


impl<Writer> Layer<Writer> {
    pub const fn new(transform: Transform2D, pixel_writer: Writer) -> Self {
        Self {
            layer_transform: transform,
            pixel_writer,
            windows: BTreeMap::new(),
        }
    }


    pub fn add_window(&mut self, key: &'static str, window: Window<impl WindowDrawable>) {
        self.windows
            .insert(key, window.into_dyn());
    }


    pub fn update_window_transform<F>(&mut self, key: &'static str, fun: F) -> KernelResult<Transform2D>
        where
            F: FnMut(&mut Transform2D),
    {
        let window_transform = self.update(key, fun)?;

        can_updated_window_transform(&self.layer_transform, &window_transform)?;

        self.window_mut(key)?
            .set_transform(window_transform.clone());

        Ok(window_transform)
    }


    pub fn window_ref(&self, key: &'static str) -> KernelResult<&Window<Box<dyn WindowDrawable>>> {
        self.windows
            .get(key)
            .ok_or(KernelError::FailedOperateLayer(NotExistsKey))
    }


    pub fn window_mut(
        &mut self,
        key: &'static str,
    ) -> KernelResult<&mut Window<Box<dyn WindowDrawable>>> {
        self.windows
            .get_mut(key)
            .ok_or(KernelError::FailedOperateLayer(NotExistsKey))
    }


    pub fn remove_window(&mut self, key: &str) {
        self.windows.remove(key);
    }


    fn update<F>(&mut self, key: &'static str, mut fun: F) -> KernelResult<Transform2D>
        where
            F: FnMut(&mut Transform2D),
    {
        let window = self.window_mut(key).unwrap();

        let mut window_transform = window.transform_ref().clone();

        fun(&mut window_transform);

        Ok(window_transform)
    }
}


impl<Writer> Layer<Writer>
    where
        Writer: PixelFlushable,
{
    pub fn draw_all_window(&mut self) -> KernelResult {
        let windows = self.windows.values_mut();
        draw_all(
            &mut self.pixel_writer,
            &self.layer_transform.rect(),
            windows,
        )
    }


    /// このレイヤー内に存在する全てのウィンドウを描画します。
    ///
    /// * `draw_rect` - 描画する領域の制限
    pub fn draw_all_window_in_area(&mut self, draw_rect: &Rectangle<usize>) -> KernelResult {
        let windows = self
            .windows
            .values_mut()
            .filter(|window| {
                draw_rect.with_in_rect(&window.transform_ref().rect())
                    || window
                    .transform_ref()
                    .rect()
                    .with_in_rect(draw_rect)
            });

        draw_all(&mut self.pixel_writer, draw_rect, windows)
    }
}


fn draw_all<'window>(
    pixel_writer: &mut dyn PixelFlushable,
    draw_rect: &Rectangle<usize>,
    windows: impl Iterator<Item=&'window mut Window<Box<dyn WindowDrawable>>>,
) -> KernelResult {
    for window in windows {
        let transform = window.transform_ref().clone();
        window
            .drawer()
            .draw_in_area(&transform, draw_rect, pixel_writer)?;
    }

    Ok(())
}


fn can_updated_window_transform(
    layer_transform: &Transform2D,
    window_transform: &Transform2D,
) -> KernelResult {
    if layer_transform.with_in(window_transform) {
        Ok(())
    } else {
        Err(KernelError::FailedOperateLayer(
            LayerReason::WindowSizeOver(window_transform.rect()),
        ))
    }
}


#[cfg(test)]
mod tests {
    use common_lib::math::size::Size;
    use common_lib::math::vector::Vector2D;
    use common_lib::transform::builder::Transform2DBuilder;

    use crate::gop::pixel::writer::mock_buffer_pixel_writer::MockBufferPixelWriter;
    use crate::layers::layer::Layer;
    use crate::layers::window::drawers::cursor::mouse_cursor::MouseCursorDrawer;
    use crate::layers::window::Window;

    #[test]
    fn it_success_update_window_position() {
        let mut layer = new_layer();

        let update_result = layer.update_window_transform("test", |window_status| {
            window_status.set_pos(Vector2D::new(20, 20));
        });

        assert!(update_result.is_ok());

        assert!(layer
            .window_ref("test")
            .is_ok_and(|window| { window.transform_ref().pos() == Vector2D::new(20, 20) }))
    }


    #[test]
    fn it_failed_update_window_position_when_over_size() {
        let mut layer = new_layer();

        let update_result = layer.update_window_transform("test", |window_status| {
            window_status.set_pos(Vector2D::new(120, 120));
        });

        assert!(update_result.is_err());

        assert!(layer
            .window_ref("test")
            .is_ok_and(|window| { window.transform_ref().pos() == Vector2D::new(0, 0) }))
    }

    fn new_layer() -> Layer<MockBufferPixelWriter> {
        let mut layer = Layer::new(
            Transform2DBuilder::new()
                .size(Size::new(100, 100))
                .build(),
            MockBufferPixelWriter::new(100, 100),
        );
        layer.add_window(
            "test",
            Window::new(
                MouseCursorDrawer::default(),
                Transform2DBuilder::new()
                    .size(Size::new(10, 10))
                    .build(),
            ),
        );

        layer
    }
}
