use alloc::rc::Rc;
use core::cell::RefCell;

use crate::class_driver::mouse::mouse_driver_factory::MouseDriverFactory;
use crate::error::PciResult;
use crate::xhc::allocator::memory_allocatable::MemoryAllocatable;
use crate::xhc::device_manager::device::device_map::DeviceMap;
use crate::xhc::device_manager::DeviceManager;
use crate::xhc::registers::traits::doorbell_registers_accessible::DoorbellRegistersAccessible;
use crate::xhc::registers::traits::port_registers_accessible::PortRegistersAccessible;
use crate::xhc::transfer::device_context::DeviceContextArrayPtr;
use crate::xhc::transfer::device_context::scratchpad_buffers_array_ptr::ScratchpadBuffersArrayPtr;

pub trait DeviceContextBaseAddressArrayPointerAccessible {
    fn write_device_context_array_addr(&mut self, device_context_addr: u64) -> PciResult;

    fn setup_device_context_array(
        &mut self,
        device_slots: u8,
        scratchpad_buffers_len: usize,
        allocator: &mut impl MemoryAllocatable,
    ) -> PciResult<DeviceContextArrayPtr> {
        let device_context_array_addr =
            allocator.try_allocate_device_context_array(device_slots + 1)?;

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

pub(crate) fn setup_device_manager<T, M>(
    registers: &mut Rc<RefCell<T>>,
    device_slots: u8,
    scratchpad_buffers_len: usize,
    allocator: &mut impl MemoryAllocatable,
    mouse_driver_factory: MouseDriverFactory,
) -> PciResult<DeviceManager<T, M>>
    where
        M: MemoryAllocatable,
        T: DeviceContextBaseAddressArrayPointerAccessible
        + DoorbellRegistersAccessible
        + PortRegistersAccessible
        + 'static,
{
    let device_context_array = registers
        .borrow_mut()
        .setup_device_context_array(device_slots, scratchpad_buffers_len, allocator)?;

    Ok(DeviceManager::new(
        DeviceMap::default(),
        device_context_array,
        registers,
        mouse_driver_factory,
    ))
}
