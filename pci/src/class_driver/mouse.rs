use core::cmp::max;

use common_lib::vector::Vector2D;
use kernel_lib::gop::console::draw_cursor;
use kernel_lib::gop::pixel::pixel_color::PixelColor;

use crate::class_driver::ClassDriver;
use crate::error::{PciError, PciResult};

const MOUSE_DATA_BUFF_SIZE: usize = 3;

pub struct Mouse {
    data_buff: [i8; MOUSE_DATA_BUFF_SIZE],
    current_pos: Vector2D<usize>,
}

impl ClassDriver for Mouse {
    fn on_data_received(&mut self) -> PciResult {
        if self.data_buff.iter().all(|b| *b == 0) {
            return Ok(());
        }

        draw(self.current_pos, PixelColor::black())?;

        let relative = cursor_pos(&self.data_buff);
        self.current_pos = Vector2D::new(
            max(self.current_pos.x() as isize + relative.x(), 0) as usize,
            max(self.current_pos.y() as isize + relative.y(), 0) as usize,
        );

        draw(self.current_pos, PixelColor::new(0x00, 0xFF, 0x00))?;

        Ok(())
    }

    fn data_buff_addr(&self) -> u64 {
        self.data_buff.as_ptr() as u64
    }

    fn data_buff_len(&self) -> u32 {
        3
    }
}

impl Mouse {
    pub fn new() -> Self {
        Self {
            current_pos: Vector2D::new(0, 0),
            data_buff: [0; MOUSE_DATA_BUFF_SIZE],
        }
    }

    pub fn data_buff_addr(&self) -> u64 {
        self.data_buff.as_ptr() as u64
    }
}

fn draw(pos: Vector2D<usize>, color: PixelColor) -> PciResult {
    draw_cursor(pos, color).map_err(|_| PciError::NullPointer)
}

fn cursor_pos(buff: &[i8]) -> Vector2D<isize> {
    Vector2D::new(buff[1] as isize, buff[2] as isize)
}
