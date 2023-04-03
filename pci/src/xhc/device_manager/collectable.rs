use crate::error::{DeviceReason, PciError, PciResult};
use crate::xhc::allocator::memory_allocatable::MemoryAllocatable;
use crate::xhc::device_manager::device_slot::Device;
use crate::xhc::registers::traits::doorbell_registers_accessible::DoorbellRegistersAccessible;
use alloc::rc::Rc;
use core::cell::RefCell;

pub mod single_device_collector;

pub trait DeviceCollectable<T>
where
    T: DoorbellRegistersAccessible,
{
    fn new(slot_id: u8) -> Self;
    /// 指定したスロットのIDをもつデバイスを取得します。
    fn mut_at(&mut self, slot_id: u8) -> Option<&mut Device<T>>;

    /// 指定したスロットIDのデバイスを作成します。
    fn set(&mut self, device_slot: Device<T>) -> PciResult;

    fn new_set(
        &mut self,
        parent_hub_slot_id: u8,
        port_speed: u8,
        slot_id: u8,
        allocator: &mut impl MemoryAllocatable,
        doorbell: &Rc<RefCell<T>>,
    ) -> PciResult<&mut Device<T>> {
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
