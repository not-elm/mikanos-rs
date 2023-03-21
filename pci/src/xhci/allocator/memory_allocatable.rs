use crate::xhci::allocator::aligned_address::AlignedAddress;

/// デバイスコンテキストの配列などのメモリを確保する際に使います。
pub trait MemoryAllocatable {
    /// 指定されたバイト数のメモリを確保し、確保先の先頭アドレスを返します。
    ///
    /// Note: このメソッドの戻り値となるアドレスは64Bytesにアラインされている必要があります。
    unsafe fn allocate_with_align(
        &mut self,
        bytes: usize,
        align: usize,
        bounds: usize,
    ) -> Option<AlignedAddress>;

    unsafe fn free(&mut self, base_addr: usize);
}
