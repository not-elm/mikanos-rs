#[repr(transparent)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct MemoryMappedAddr(usize);

impl MemoryMappedAddr {
    pub(crate) fn new(mmio_addr: usize) -> Self {
        Self(mmio_addr)
    }

    pub fn addr(&self) -> usize {
        self.0
    }
}
