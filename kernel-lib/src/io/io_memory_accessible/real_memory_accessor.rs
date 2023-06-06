use crate::io::asm::{io_in32, io_out32};
use crate::io::io_memory_accessible::IoMemoryAccessible;

#[derive(Debug, Clone)]
pub struct RealIoMemoryAccessor {}


impl RealIoMemoryAccessor {
    pub const fn new() -> Self {
        Self {}
    }
}

impl IoMemoryAccessible for RealIoMemoryAccessor {
    fn io_in(&self, port: u16) -> u32 {
        io_in32(port)
    }

    fn io_out(&mut self, port: u16, value: u32) {
        io_out32(port, value)
    }
}
