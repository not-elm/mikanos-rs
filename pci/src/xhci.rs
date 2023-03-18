use allocator::memory_allocatable::MemoryAllocatable;
use kernel_lib::println;

use crate::error::{PciError, PciResult};
use crate::xhci::registers::capability_registers::structural_parameters1::number_of_device_slots::NumberOfDeviceSlots;
use crate::xhci::registers::operational_registers::config_register::max_device_slots_enabled::MaxDeviceSlotsEnabled;
use crate::xhci::registers::operational_registers::device_context_base_address_array_pointer::DeviceContextBaseAddressArrayPointer;
use crate::xhci::registers::operational_registers::usb_command_register::host_controller_reset::HostControllerReset;
use crate::xhci::registers::operational_registers::usb_command_register::run_stop::RunStop;
use crate::xhci::registers::operational_registers::usb_status_register::controller_not_ready::ControllerNotReady;
use crate::xhci::registers::operational_registers::usb_status_register::host_controller_halted::HostControllerHalted;

pub mod allocator;
pub mod registers;

///
/// 1. xhcのリセット
/// 2. デバイスコンテキストの設定
pub fn _init() {}

/// 接続できるデバイス数を取得して、コンフィグレジスタに設定します。
pub fn set_device_context(
    run_stop: &RunStop,
    max_slots: &NumberOfDeviceSlots,
    max_slots_en: &MaxDeviceSlotsEnabled,
) -> PciResult {
    if run_stop.read_flag_volatile() {
        return Err(PciError::XhcRunning);
    }
    let enable_slots = max_slots.read_volatile();
    max_slots_en.write_volatile(enable_slots);

    if max_slots.read_volatile() == enable_slots {
        Ok(())
    } else {
        Err(PciError::FailedWroteSetMaxSlotsEn(enable_slots))
    }
}

pub unsafe fn allocate_device_context_array(
    dcbaap: &DeviceContextBaseAddressArrayPointer,
    max_slots_en: &MaxDeviceSlotsEnabled,
    allocator: &mut impl MemoryAllocatable,
) -> PciResult {
    const DEVICE_CONTEXT_SIZE: usize = 1024;

    let alloc_size = DEVICE_CONTEXT_SIZE * (max_slots_en.read_volatile() + 1) as usize;
    let device_context_array_addr = allocator
        .alloc(alloc_size)
        .ok_or(PciError::FailedAllocate)?;
    dcbaap.write_volatile(device_context_array_addr as u64);

    let addr = dcbaap.read_volatile();
    if addr == device_context_array_addr as u64 {
        Ok(())
    } else {
        Err(PciError::WriteFailedDeviceContextArrayAddrToDCBAAP(addr))
    }
}

pub fn reset_controller(
    hch: &HostControllerHalted,
    hcrst: &HostControllerReset,
    cnr: &ControllerNotReady,
) -> PciResult {
    if !hch.read_flag_volatile() {
        return Err(PciError::HostControllerNotHalted);
    }
    println!("start write true -> host controller reset");

    hcrst.reset();
    println!("write true -> host controller reset");

    cnr.wait_until_ready();
    println!(
        "controller is ready! current is = {}",
        cnr.read_flag_volatile()
    );

    Ok(())
}
