use alloc::vec::Vec;

use crate::io::io_memory_accessible::IoMemoryAccessible;

/// TEST用のモックI/O空間アクセッサーです
pub struct MockMemoryAccessor {
    base_addr: usize,

    buff: Vec<u32>,
}

impl MockMemoryAccessor {
    pub const fn new(base_addr: usize, buff: Vec<u32>) -> Self {
        Self { base_addr, buff }
    }
}

impl MockMemoryAccessor {
    fn index(&self, port: u16) -> usize {
        let addr = port as usize - self.base_addr;
        addr / core::mem::size_of::<u32>()
    }
}

impl IoMemoryAccessible for MockMemoryAccessor {
    fn io_in(&self, port: u16) -> u32 {
        self.buff[self.index(port)]
    }

    fn io_out(&mut self, port: u16, value: u32) {
        let index = self.index(port);
        self.buff[index] = value;
    }
}

#[cfg(test)]
mod tests {
    use alloc::vec;

    use crate::configuration_space::io::io_memory_accessible::mock_memory_accessor::MockMemoryAccessor;
    use crate::io::io_memory_accessible::mock_memory_accessor::MockMemoryAccessor;

    #[test]
    fn it_index_is_1() {
        let index = MockMemoryAccessor::new(0, vec![0, 0, 0]).index(0x04);
        assert_eq!(index, 1);
    }

    #[test]
    fn it_index_is_2() {
        let index = MockMemoryAccessor::new(0, vec![0, 0, 0]).index(0x08);
        assert_eq!(index, 2);
    }
}
