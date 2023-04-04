use crate::class_driver::mouse::{MouseButton, MOUSE_DATA_BUFF_SIZE};
use crate::class_driver::ClassDriverOperate;
use crate::error::PciResult;
use common_lib::vector::Vector2D;

pub trait MouseSubscriber {
    fn subscribe(
        &mut self,
        prev_cursor: Vector2D<usize>,
        current_cursor: Vector2D<usize>,
        button: MouseButton,
    );
}
pub struct MouseSubscribeDriver<T>
where
    T: MouseSubscriber,
{
    data_buff: [i8; MOUSE_DATA_BUFF_SIZE],
    current_pos: Vector2D<usize>,
    subscriber: T,
}
impl<T> ClassDriverOperate for MouseSubscribeDriver<T>
where
    T: MouseSubscriber,
{
    fn on_data_received(&mut self) -> PciResult {
        if self.data_buff.iter().all(|b| *b == 0) {
            return Ok(());
        }

        let prev_cursor = self.current_pos.clone();
        self.subscriber
            .subscribe(prev_cursor, self.current_pos, MouseButton::Left);

        //TODO BUTTON

        Ok(())
    }

    fn data_buff_addr(&self) -> u64 {
        self.data_buff.as_ptr() as u64
    }

    fn data_buff_len(&self) -> u32 {
        3
    }
}

impl<T> MouseSubscribeDriver<T>
where
    T: MouseSubscriber,
{
    pub fn new(subscriber: T) -> Self {
        Self {
            current_pos: Vector2D::new(0, 0),
            data_buff: [0; MOUSE_DATA_BUFF_SIZE],
            subscriber,
        }
    }

    pub fn data_buff_addr(&self) -> u64 {
        self.data_buff.as_ptr() as u64
    }
}
