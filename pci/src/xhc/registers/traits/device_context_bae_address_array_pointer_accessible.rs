use crate::error::PciResult;
use crate::xhc::allocator::memory_allocatable::MemoryAllocatable;

pub trait DeviceContextBaseAddressArrayPointerAccessible {
    fn write_device_context_array_addr(&mut self, device_context_addr: u64) -> PciResult;

    fn setup_device_context_array(&mut self, allocator: &mut impl MemoryAllocatable) -> PciResult {
        let device_context_array_addr = allocator.try_allocate_device_context_array(8)?;
        self.write_device_context_array_addr(device_context_array_addr)?;
        Ok(())
    }
}
