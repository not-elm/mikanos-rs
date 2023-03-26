use crate::error::PciResult;
use crate::xhc::allocator::memory_allocatable::MemoryAllocatable;

pub struct DeviceContextArray {}

impl DeviceContextArray {
    pub fn new_with_alloc(allocator: &mut impl MemoryAllocatable) -> PciResult<Self> {
        todo!();
        //  let device_context_array_address =
        //  xhci::context::Device64Byte::new_64byte()
        // unsafe{
        //      allocator.allocate_with_align()
        //  }
    }
}
