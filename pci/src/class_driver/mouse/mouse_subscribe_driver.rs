use alloc::boxed::Box;

use common_lib::math::vector::Vector2D;

use crate::class_driver::boot_protocol_buffer::BootProtocolBuffer;
use crate::class_driver::ClassDriverOperate;
use crate::class_driver::mouse::{
    current_cursor_pos, mouse_button_boot_protocol, MOUSE_DATA_BUFF_SIZE,
};
use crate::class_driver::mouse::mouse_subscribable::MouseSubscribable;
use crate::error::{PciError, PciResult};

pub struct MouseSubscribeDriver {
    data_buff: [i8; MOUSE_DATA_BUFF_SIZE],
    current_pos: Vector2D<usize>,
    subscriber: Box<dyn MouseSubscribable>,
}

impl ClassDriverOperate for MouseSubscribeDriver {
    fn on_data_received(&mut self) -> PciResult {
        if self
            .data_buff
            .iter()
            .all(|b| *b == 0)
        {
            return Ok(());
        }

        let prev_cursor = self.current_pos;
        self.current_pos = current_cursor_pos(prev_cursor, &self.data_buff);
        self.subscriber
            .subscribe(
                prev_cursor,
                self.current_pos,
                mouse_button_boot_protocol(BootProtocolBuffer::new(&self.data_buff)),
            )
            .map_err(|_| PciError::UserError)?;

        Ok(())
    }

    fn data_buff_addr(&self) -> u64 {
        self.data_buff.as_ptr() as u64
    }

    fn data_buff_len(&self) -> u32 {
        MOUSE_DATA_BUFF_SIZE as u32
    }
}

impl MouseSubscribeDriver {
    pub fn new(subscriber: Box<dyn MouseSubscribable>) -> Self {
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
