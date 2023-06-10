use common_lib::math::vector::Vector2D;

use crate::class_driver::mouse::MouseButton;

pub trait MouseSubscribable {
    /// Performs user-defined processing based on previous and current mouse
    /// cursor
    ///
    /// This Function is called whenever a mouse action occurs.
    fn subscribe(
        &self,
        prev_cursor: Vector2D<usize>,
        current_cursor: Vector2D<usize>,
        prev_button: Option<MouseButton>,
        button: Option<MouseButton>,
    ) -> anyhow::Result<()>;
}


impl<T> MouseSubscribable for T
where
    T: Fn(
        Vector2D<usize>,
        Vector2D<usize>,
        Option<MouseButton>,
        Option<MouseButton>,
    ) -> anyhow::Result<()>,
{
    fn subscribe(
        &self,
        prev_cursor: Vector2D<usize>,
        current_cursor: Vector2D<usize>,
        prev_button: Option<MouseButton>,
        button: Option<MouseButton>,
    ) -> anyhow::Result<()> {
        self(prev_cursor, current_cursor, prev_button, button)
    }
}
