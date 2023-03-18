use allocator::memory_allocatable::MemoryAllocatable;
use kernel_lib::println;

use crate::error::OperationReason::{FailedAllocate, NotReflectedValue};
use crate::error::{OperationReason, PciError, PciResult};
use crate::xhci::registers::capability_registers::structural_parameters1::number_of_device_slots::NumberOfDeviceSlots;
use crate::xhci::registers::operational_registers::command_ring_control_register::CommandRingControlRegister;
use crate::xhci::registers::operational_registers::config_register::max_device_slots_enabled::MaxDeviceSlotsEnabled;
use crate::xhci::registers::operational_registers::device_context_base_address_array_pointer::DeviceContextBaseAddressArrayPointer;
use crate::xhci::registers::operational_registers::usb_command_register::host_controller_reset::HostControllerReset;
use crate::xhci::registers::operational_registers::usb_command_register::run_stop::RunStop;
use crate::xhci::registers::operational_registers::usb_status_register::controller_not_ready::ControllerNotReady;
use crate::xhci::registers::operational_registers::usb_status_register::host_controller_halted::HostControllerHalted;
use crate::VolatileAccessible;

pub mod allocator;
pub mod registers;

///
/// 1. xhcのリセット
/// 2. デバイスコンテキストの設定
pub fn init() -> PciResult {
    // reset_controller()?;
    // set_device_context()?;
    // allocate_device_context_array()?
    // USBCOMMAND RUN
    Ok(())
}

pub fn reset_controller(
    hch: &HostControllerHalted,
    run_stop: &RunStop,
    hcrst: &HostControllerReset,
    cnr: &ControllerNotReady,
) -> PciResult {
    if !hch.read_flag_volatile() {
        run_stop.write_flag_volatile(false);
    }
    hch.until_halted();
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

/// 接続できるデバイス数を取得して、コンフィグレジスタに設定します。
pub fn set_device_context(
    run_stop: &RunStop,
    max_slots: &NumberOfDeviceSlots,
    max_slots_en: &MaxDeviceSlotsEnabled,
) -> PciResult {
    if run_stop.read_flag_volatile() {
        return Err(PciError::FailedOperateToRegister(
            OperationReason::XhcRunning,
        ));
    }
    let enable_slots = max_slots.read_volatile();
    max_slots_en.write_volatile(enable_slots);

    if max_slots.read_volatile() == enable_slots {
        Ok(())
    } else {
        Err(PciError::FailedOperateToRegister(
            OperationReason::NotReflectedValue {
                value: enable_slots as usize,
            },
        ))
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
        .ok_or(PciError::FailedOperateToRegister(FailedAllocate))?;
    dcbaap.write_volatile(device_context_array_addr as u64);

    let addr = dcbaap.read_volatile();
    if addr == device_context_array_addr as u64 {
        Ok(())
    } else {
        Err(PciError::FailedOperateToRegister(NotReflectedValue {
            value: addr as usize,
        }))
    }
}

pub unsafe fn allocate_command_ring(
    crcr: &CommandRingControlRegister,
    allocator: &mut impl MemoryAllocatable,
) -> PciResult {
    const TRB_SIZE: usize = 128;

    let alloc_size = TRB_SIZE * 32;
    let command_ring_addr = allocator
        .alloc(alloc_size)
        .ok_or(PciError::FailedOperateToRegister(FailedAllocate))?;

    register_command_ring(crcr, command_ring_addr as u64)
}

pub fn run(crcr: &CommandRingControlRegister, command_ring_addr: u64) -> PciResult {
    if crcr.cs.read_flag_volatile() || crcr.ca.read_flag_volatile() {
        return Err(PciError::FailedOperateToRegister(
            OperationReason::MustBeCommandRingStopped,
        ));
    }
    crcr.rcs.write_flag_volatile(true);
    crcr.command_ring_pointer
        .set_command_ring_addr(command_ring_addr);
    Ok(())
}

fn register_command_ring(crcr: &CommandRingControlRegister, command_ring_addr: u64) -> PciResult {
    if crcr.cs.read_flag_volatile() || crcr.ca.read_flag_volatile() {
        return Err(PciError::FailedOperateToRegister(
            OperationReason::MustBeCommandRingStopped,
        ));
    }
    crcr.rcs.write_flag_volatile(true);
    crcr.command_ring_pointer
        .set_command_ring_addr(command_ring_addr);
    Ok(())
}
