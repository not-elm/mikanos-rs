use pci::error::PciResult;
use pci::xhci::allocator::mikanos_pci_memory_allocator::MikanOSPciMemoryAllocator;
use pci::xhci::registers::capability_registers::structural_parameters1::number_of_device_slots::NumberOfDeviceSlots;
use pci::xhci::registers::operational_registers::command_ring_control_register::CommandRingControlRegister;
use pci::xhci::registers::operational_registers::config_register::max_device_slots_enabled::MaxDeviceSlotsEnabled;
use pci::xhci::registers::operational_registers::device_context_base_address_array_pointer::DeviceContextBaseAddressArrayPointer;
use pci::xhci::registers::operational_registers::usb_command_register::host_controller_reset::HostControllerReset;
use pci::xhci::registers::operational_registers::usb_command_register::run_stop::RunStop;
use pci::xhci::registers::operational_registers::usb_command_register::usb_command_register_field::UsbCommandRegisterField;
use pci::xhci::registers::operational_registers::usb_status_register::controller_not_ready::ControllerNotReady;
use pci::xhci::registers::operational_registers::usb_status_register::host_controller_halted::HostControllerHalted;
use pci::xhci::registers::operational_registers::usb_status_register::usb_status_register_field::UsbStatusRegisterField;
use pci::xhci::{allocate_command_ring, allocate_device_context_array, reset_controller};
use pci::VolatileAccessible;

use crate::hcs1_offset;
use crate::test_runner::xhci::{
    command_ring_control_register_offset, config_register_offset, dcbaap_offset,
    operation_registers_offset, usb_status_register_offset,
};

#[test_case]
fn it_reset_xhc_host_controller() {
    execute_reset_host_controller().unwrap();
}

#[test_case]
fn it_set_device_contexts_enabled() {
    set_device_context_max_slot().unwrap();
}

#[test_case]
fn it_allocate_device_context_array_address_and_set_to_dcbaap() {
    execute_reset_host_controller().unwrap();
    set_device_context_max_slot().unwrap();

    unsafe {
        allocate_device_context_array(
            &DeviceContextBaseAddressArrayPointer::new(dcbaap_offset()),
            &MaxDeviceSlotsEnabled::new(config_register_offset()),
            &mut MikanOSPciMemoryAllocator::new(),
        )
        .unwrap();
    }
}

#[test_case]
fn it_allocate_command_ring() {
    execute_reset_host_controller().unwrap();
    set_device_context_max_slot().unwrap();
    unsafe {
        allocate_command_ring(
            &CommandRingControlRegister::new(command_ring_control_register_offset()).unwrap(),
            &mut MikanOSPciMemoryAllocator::new(),
        )
        .unwrap()
    }
}

pub(crate) fn execute_reset_host_controller() -> PciResult {
    reset_controller(
        &HostControllerHalted::new(usb_status_register_offset()),
        &RunStop::new(operation_registers_offset()),
        &HostControllerReset::new(operation_registers_offset()),
        &ControllerNotReady::new(usb_status_register_offset()),
    )?;

    RunStop::new(operation_registers_offset()).write_flag_volatile(false);
    Ok(())
}

fn set_device_context_max_slot() -> PciResult {
    let run_stop = RunStop::new(operation_registers_offset());
    let max_slots = NumberOfDeviceSlots::new(hcs1_offset());
    let max_slots_en = MaxDeviceSlotsEnabled::new(config_register_offset());
    pci::xhci::set_device_context(&run_stop, &max_slots, &max_slots_en)
}
