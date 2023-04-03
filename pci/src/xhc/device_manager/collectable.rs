use alloc::rc::Rc;
use core::cell::RefCell;

use crate::error::{DeviceReason, PciError, PciResult};
use crate::xhc::allocator::memory_allocatable::MemoryAllocatable;
use crate::xhc::device_manager::device::Device;
use crate::xhc::registers::traits::doorbell_registers_accessible::DoorbellRegistersAccessible;

pub mod single_device_collector;

pub trait DeviceCollectable<T, Memory>
where
    T: DoorbellRegistersAccessible,
    Memory: MemoryAllocatable,
{
    fn new(slot_id: u8) -> Self;
    /// 指定したスロットのIDをもつデバイスを取得します。
    fn mut_at(&mut self, slot_id: u8) -> Option<&mut Device<T, Memory>>;

    /// 指定したスロットIDのデバイスを作成します。
    fn set(&mut self, device_slot: Device<T, Memory>) -> PciResult;

    fn new_set(
        &mut self,
        parent_hub_slot_id: u8,
        port_speed: u8,
        slot_id: u8,
        allocator: &Rc<RefCell<Memory>>,
        doorbell: &Rc<RefCell<T>>,
    ) -> PciResult<&mut Device<T, Memory>> {
        self.set(Device::new_with_init_default_control_pipe(
            parent_hub_slot_id,
            port_speed,
            slot_id,
            allocator,
            doorbell,
        )?)?;

        self.mut_at(slot_id)
            .ok_or(PciError::FailedOperateDevice(DeviceReason::NotExistsSlot(
                slot_id,
            )))
    }
}
