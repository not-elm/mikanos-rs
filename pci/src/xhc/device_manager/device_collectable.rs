use crate::error::PciResult;
use crate::xhc::allocator::memory_allocatable::MemoryAllocatable;
use crate::xhc::device_manager::device::Device;
use crate::xhc::registers::traits::doorbell_registers_accessible::DoorbellRegistersAccessible;

pub mod single_device_collector;

pub trait DeviceCollectable {
    fn new(slot_id: u8) -> Self;
    /// 指定したスロットのIDをもつデバイスを取得します。
    fn mut_at(&mut self, slot_id: u8) -> Option<&mut Device>;

    /// 指定したスロットIDのデバイスを作成します。
    fn new_set_at(
        &mut self,
        parent_hub_slot_id: u8,
        port_speed: u8,
        slot_id: u8,
        allocator: &mut impl MemoryAllocatable,
    ) -> PciResult;
}
