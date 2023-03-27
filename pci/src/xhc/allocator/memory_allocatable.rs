use crate::error::{AllocateReason, PciError, PciResult};
use crate::xhc::allocator::aligned_address::AlignedAddress;

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

    fn try_allocate_trb_ring(&mut self, ring_size: usize) -> PciResult<u64> {
        unsafe {
            self.try_allocate_with_align(core::mem::size_of::<u128>() * ring_size, 64, 64 * 1024)?
                .address()
        }
    }

    unsafe fn try_allocate_with_align(
        &mut self,
        bytes: usize,
        align: usize,
        bounds: usize,
    ) -> PciResult<AlignedAddress> {
        self.allocate_with_align(bytes, align, bounds)
            .ok_or(PciError::FailedAllocate(AllocateReason::NotEnoughMemory))
    }

    unsafe fn free(&mut self, base_addr: usize);
}
