use alloc::rc::Rc;

use common_lib::math::vector::Vector2D;

use crate::class_driver::boot_protocol_buffer::BootProtocolBuffer;
use crate::class_driver::mouse::subscribable::MouseSubscribable;
use crate::class_driver::mouse::{
    current_cursor_pos, mouse_button_boot_protocol, MouseButton, MOUSE_DATA_BUFF_SIZE,
};
use crate::class_driver::ClassDriverOperate;
use crate::error::PciResult;
use crate::pci_error;

#[derive(Clone)]
pub struct MouseDriver {
    data_buff: [i8; MOUSE_DATA_BUFF_SIZE],
    current_pos: Vector2D<usize>,
    current_button: Option<MouseButton>,
    subscriber: Rc<dyn MouseSubscribable>,
}


impl ClassDriverOperate for MouseDriver {
    fn on_data_received(&mut self) -> PciResult {
        if self
            .data_buff
            .iter()
            .all(|b| *b == 0)
        {
            return Ok(());
        }

        let prev_cursor = self.current_pos;
        let prev_button = self.current_button;
        self.current_button = mouse_button_boot_protocol(BootProtocolBuffer::new(&self.data_buff));

        self.current_pos = current_cursor_pos(prev_cursor, &self.data_buff);
        self.subscriber
            .subscribe(
                prev_cursor,
                self.current_pos,
                prev_button,
                self.current_button,
            )
            .map_err(|e| pci_error!("{e:?}"))?;

        Ok(())
    }


    fn data_buff_addr(&self) -> u64 {
        self.data_buff.as_ptr() as u64
    }


    fn data_buff_len(&self) -> u32 {
        MOUSE_DATA_BUFF_SIZE as u32
    }
}


impl MouseDriver {
    pub fn new(subscriber: impl MouseSubscribable + 'static) -> Self {
        Self {
            current_pos: Vector2D::new(0, 0),
            data_buff: [0; MOUSE_DATA_BUFF_SIZE],
            current_button: None,
            subscriber: Rc::new(subscriber),
        }
    }


    pub fn data_buff_addr(&self) -> u64 {
        self.data_buff.as_ptr() as u64
    }
}
