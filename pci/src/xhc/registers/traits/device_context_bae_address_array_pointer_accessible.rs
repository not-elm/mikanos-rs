use crate::error::PciResult;
use crate::xhc::allocator::memory_allocatable::MemoryAllocatable;
use crate::xhc::device_manager::device_collectable::single_device_collector::SingleDeviceCollector;
use crate::xhc::device_manager::DeviceManager;
use crate::xhc::transfer::device_context::scratchpad_buffers_array_ptr::ScratchpadBuffersArrayPtr;
use crate::xhc::transfer::device_context::DeviceContextArrayPtr;

pub trait DeviceContextBaseAddressArrayPointerAccessible {
    fn write_device_context_array_addr(&mut self, device_context_addr: u64) -> PciResult;

    fn setup_device_manager(
        &mut self,
        device_slots: u8,
        scratchpad_buffers_len: usize,
        allocator: &mut impl MemoryAllocatable,
    ) -> PciResult<DeviceManager<SingleDeviceCollector>> {
        let device_context_array =
            self.setup_device_context_array(device_slots, scratchpad_buffers_len, allocator)?;
        Ok(DeviceManager::new(
            SingleDeviceCollector::new(device_slots),
            device_context_array,
        ))
    }

    fn setup_device_context_array(
        &mut self,
        device_slots: u8,
        scratchpad_buffers_len: usize,
        allocator: &mut impl MemoryAllocatable,
    ) -> PciResult<DeviceContextArrayPtr> {
        let device_context_array_addr =
            allocator.try_allocate_device_context_array(device_slots)?;
        let mut device_context_array = DeviceContextArrayPtr::new(device_context_array_addr);

        if 0 < scratchpad_buffers_len {
            let scratchpad_buffers_array =
                ScratchpadBuffersArrayPtr::new(scratchpad_buffers_len, allocator)?;
            device_context_array.set_device_context_at(0, scratchpad_buffers_array.base_addr());
        }

        self.write_device_context_array_addr(device_context_array_addr)?;
        Ok(device_context_array)
    }
}
