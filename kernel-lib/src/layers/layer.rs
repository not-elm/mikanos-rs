use alloc::boxed::Box;
use alloc::collections::BTreeMap;

use crate::error::LayerReason::NotExistsKey;
use crate::error::{KernelError, KernelResult};
use crate::gop::pixel::pixel_writable::PixelWritable;
use crate::layers::layer_status::{check_window_status, LayerStatus};
use crate::layers::window::drawers::WindowDrawable;
use crate::layers::window::status::WindowStatus;
use crate::layers::window::Window;

pub struct Layer<Writer> {
    layer_status: LayerStatus,
    pixel_writer: Writer,
    windows: BTreeMap<&'static str, Window<Box<dyn WindowDrawable>>>,
}


impl<Writer> Layer<Writer> {
    pub const fn new(status: LayerStatus, writer: Writer) -> Self {
        Self {
            layer_status: status,
            pixel_writer: writer,
            windows: BTreeMap::new(),
        }
    }


    pub fn add_window(&mut self, key: &'static str, window: Window<impl WindowDrawable>) {
        self.windows
            .insert(key, window.into_dyn());
    }


    pub fn update_window<F>(&mut self, key: &'static str, fun: F) -> KernelResult
    where
        F: FnMut(&mut WindowStatus),
    {
        let update_status = self.update(key, fun)?;


        check_window_status(&self.layer_status, &update_status)?;

        self.window_mut(key)?
            .set_window_status(update_status);

        Ok(())
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


    fn update<F>(&mut self, key: &'static str, mut fun: F) -> KernelResult<WindowStatus>
    where
        F: FnMut(&mut WindowStatus),
    {
        let window = self.window_mut(key)?;

        let mut update_status = window.status_ref().clone();

        fun(&mut update_status);

        Ok(update_status)
    }
}


impl<Writer> Layer<Writer>
where
    Writer: PixelWritable,
{
    pub fn draw_all(&mut self) -> KernelResult {
        for window in self.windows.values_mut() {
            let status = window.status_ref().clone();
            window
                .drawer()
                .draw(&status, &mut self.pixel_writer)?;
        }

        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use common_lib::math::size::Size;
    use common_lib::math::vector::Vector2D;

    use crate::gop::pixel::mock_buffer_pixel_writer::MockBufferPixelWriter;
    use crate::layers::layer::Layer;
    use crate::layers::layer_status::LayerStatus;
    use crate::layers::window::drawers::mouse_cursor::MouseCursorDrawer;
    use crate::layers::window::status::builder::WindowStatusBuilder;
    use crate::layers::window::Window;

    #[test]
    fn it_success_update_window_position() {
        let mut layer = new_layer();

        let update_result = layer.update_window("test", |window_status| {
            window_status.set_pos(Vector2D::new(20, 20));
        });

        assert!(update_result.is_ok());

        assert!(layer
            .window_ref("test")
            .is_ok_and(|window| { window.status_ref().pos() == Vector2D::new(20, 20) }))
    }


    #[test]
    fn it_failed_update_window_position_when_over_size() {
        let mut layer = new_layer();

        let update_result = layer.update_window("test", |window_status| {
            window_status.set_pos(Vector2D::new(120, 120));
        });

        assert!(update_result.is_err());

        assert!(layer
            .window_ref("test")
            .is_ok_and(|window| { window.status_ref().pos() == Vector2D::new(0, 0) }))
    }

    fn new_layer() -> Layer<MockBufferPixelWriter> {
        let mut layer = Layer::new(
            LayerStatus::new(Size::new(100, 100)),
            MockBufferPixelWriter::new(100, 100),
        );
        layer.add_window(
            "test",
            Window::new(
                MouseCursorDrawer::new(Vector2D::new(1, 1)),
                WindowStatusBuilder::new()
                    .size(Size::new(10, 10))
                    .build(),
            ),
        );

        layer
    }
}
