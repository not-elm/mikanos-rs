use common_lib::math::vector::Vector2D;

use crate::class_driver::mouse::MOUSE_DATA_BUFF_SIZE;
use crate::class_driver::ClassDriverOperate;
use crate::error::PciResult;

pub struct MouseDefaultDriver {
    data_buff: [i8; MOUSE_DATA_BUFF_SIZE],
    _current_pos: Vector2D<usize>,
}

impl ClassDriverOperate for MouseDefaultDriver {
    fn on_data_received(&mut self) -> PciResult {
        // if self
        //     .data_buff
        //     .iter()
        //     .all(|b| *b == 0)
        // {
        //     return Ok(());
        // }
        //
        // draw(self.current_pos, PixelColor::black())?;
        //
        // let relative = cursor_pos(&self.data_buff);
        // self.current_pos = Vector2D::new(
        //     max(self.current_pos.x() as isize + relative.x(), 0) as usize,
        //     max(self.current_pos.y() as isize + relative.y(), 0) as usize,
        // );
        //
        // draw(self.current_pos, PixelColor::new(0x00, 0xFF, 0x00))?;

        Ok(())
    }

    fn data_buff_addr(&self) -> u64 {
        self.data_buff.as_ptr() as u64
    }

    fn data_buff_len(&self) -> u32 {
        3
    }
}

impl MouseDefaultDriver {
    pub fn new() -> Self {
        Self {
            _current_pos: Vector2D::new(0, 0),
            data_buff: [0; MOUSE_DATA_BUFF_SIZE],
        }
    }
}
