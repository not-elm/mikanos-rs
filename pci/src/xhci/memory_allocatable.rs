/// デバイスコンテキストの配列などのメモリを確保する際に使います。
pub trait MemoryAllocatable {
    unsafe fn alloc(&mut self, bytes: usize) -> Option<usize>;

    unsafe fn free(&mut self, base_addr: usize);
}
