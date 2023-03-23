pub(crate) static KERNEL_MAIN_STACK: crate::entry_point::KernelMainStack =
    crate::entry_point::KernelMainStack::new();

#[repr(C, align(16))]
pub(crate) struct KernelMainStack([u8; 1024 * 1024]);

impl KernelMainStack {
    pub const fn new() -> Self {
        Self([0u8; 1024 * 1024])
    }
    pub fn end_addr(&self) -> usize {
        self.0.as_ptr().addr() + self.0.len()
    }
}
