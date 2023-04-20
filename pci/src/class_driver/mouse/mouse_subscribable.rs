use dyn_clone::DynClone;

use common_lib::math::vector::Vector2D;

use crate::class_driver::mouse::MouseButton;

/// 前回と現在のマウスカーソルの座標を元にユーザー定義の処理を行います。
///
/// このトレイトがMouseSubscribeDriverに登録されている場合、
/// マウスの状態の変更が検知されるたびにこのトレイトの処理が呼び出されます。
pub trait MouseSubscribable: DynClone {
    fn subscribe(
        &mut self,
        prev_cursor: Vector2D<usize>,
        current_cursor: Vector2D<usize>,
        button: Option<MouseButton>,
    ) -> Result<(), ()>;
}
dyn_clone::clone_trait_object!(MouseSubscribable);

impl<T> MouseSubscribable for T
    where
        T: Fn(Vector2D<usize>, Vector2D<usize>, Option<MouseButton>) -> Result<(), ()> + Clone,
{
    fn subscribe(
        &mut self,
        prev_cursor: Vector2D<usize>,
        current_cursor: Vector2D<usize>,
        button: Option<MouseButton>,
    ) -> Result<(), ()> {
        self(prev_cursor, current_cursor, button)
    }
}
