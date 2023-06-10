use crate::error::PciResult;
use crate::pci_error;
use crate::xhc::allocator::aligned_address::AlignedAddress;

/// デバイスコンテキストの配列などのメモリを確保する際に使います。
pub trait MemoryAllocatable {
    /// Allocate the specified number of bytes of memory,
    /// and return the start address which allocation destination.
    ///
    /// ## Note
    ///
    /// The address which returned by this method must be aligned to 64 bytes.
    ///
    /// ## Safety
    ///
    /// Implementors of this trait must return correct and usable addresses.
    unsafe fn allocate_with_align(
        &mut self,
        bytes: usize,
        align: usize,
        bounds: usize,
    ) -> Option<AlignedAddress>;


    fn try_allocate_trb_ring(&mut self, ring_size: usize) -> PciResult<u64> {
        self.try_allocate_with_align(core::mem::size_of::<u128>() * ring_size, 64, 4096)?
            .address()
    }


    fn try_allocate_device_context_array(&mut self, max_slots: u8) -> PciResult<u64> {
        self.try_allocate_with_align(core::mem::size_of::<u64>() * max_slots as usize, 64, 4096)?
            .address()
    }


    fn try_allocate_input_context(&mut self) -> PciResult<u64> {
        self.try_allocate_with_align(core::mem::size_of::<xhci::context::Input32Byte>(), 64, 0)?
            .address()
    }


    fn try_allocate_device_context(&mut self) -> PciResult<u64> {
        self.try_allocate_with_align(core::mem::size_of::<xhci::context::Device32Byte>(), 64, 0)?
            .address()
    }


    fn try_allocate_max_scratchpad_buffers(&mut self, len: usize) -> PciResult<u64> {
        self.try_allocate_with_align(core::mem::size_of::<u64>() * len, 4096, 4096)?
            .address()
    }


    /// Attempts to allocate and returns an error if it fails.
    fn try_allocate_with_align(
        &mut self,
        bytes: usize,
        align: usize,
        bounds: usize,
    ) -> PciResult<AlignedAddress> {
        unsafe {
            self.allocate_with_align(bytes, align, bounds)
                .ok_or(pci_error!("Not enough memory"))
        }
    }


    /// Frees memory based on the specified address.
    ///
    /// ## Safety
    ///
    /// You must pass the address that provided by the methods of this trait:
    /// The number of bytes of memory to be freed must also be exact.
    unsafe fn free(&mut self, addr: u64, bytes: usize);
}
