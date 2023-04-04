use crate::class_driver::mouse::MouseButton;
use common_lib::vector::Vector2D;
use dyn_clone::DynClone;

pub trait MouseSubscribable: DynClone {
    fn subscribe(
        &mut self,
        prev_cursor: Vector2D<usize>,
        current_cursor: Vector2D<usize>,
        button: MouseButton,
    );
}
dyn_clone::clone_trait_object!(MouseSubscribable);
