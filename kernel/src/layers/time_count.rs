use common_lib::math::size::Size;
use common_lib::math::vector::Vector2D;
use common_lib::transform::transform2d::Transform2D;
use kernel_lib::error::KernelResult;
use kernel_lib::layers::count::CountLayer;
use kernel_lib::layers::layer_key::LayerKey;
use kernel_lib::layers::window::WindowLayer;

pub(crate) fn time_count_window(
    title: &str,
    pos: Vector2D<usize>,
    text_layer_key: &str,
    window_layer_key: &str,
) -> KernelResult<LayerKey> {
    let transform = Transform2D::new(pos, Size::new(160, 52));
    let count = count_layer(&transform, text_layer_key)?;
    let window = WindowLayer::new_default_color(title, transform)
        .then_add(|_| count)?
        .into_enum()
        .into_layer_key(window_layer_key);

    Ok(window)
}


fn count_layer(window_transform: &Transform2D, key: &str) -> KernelResult<LayerKey> {
    let size = window_transform.size() - Size::new(20, 0);
    let pos = Vector2D::new(
        window_transform
            .size()
            .width()
            / 2
            - 32,
        0,
    );

    let count = CountLayer::new(Transform2D::new(
        pos,
        size.unwrap_or(window_transform.size()),
    ))?;

    Ok(count
        .into_enum()
        .into_layer_key(key))
}
