use pci::xhci::allocator::mikanos_pci_memory_allocator::MikanOSPciMemoryAllocator;
use pci::xhci::registers::operational_registers::command_ring_control_register::CommandRingControlRegister;

use crate::test_runner::xhci::{
    command_ring_control_register_offset, operational_registers, registers,
};

mod capability_registers;
mod operational_registers;
mod port_register;
mod runtime_registers;
mod transfer;

#[test_case]
fn it_new_all_registers() {
    registers();
}

#[test_case]
fn it_reset_xhc_host_controller() {
    execute_reset_host_controller();
}

#[test_case]
fn it_set_device_contexts_enabled() {
    execute_reset_host_controller();
}

#[test_case]
fn it_allocate_device_context_array_address_and_set_to_dcbaap() {
    execute_reset_host_controller();
    execute_set_device_context_max_slot();
    execute_allocate_device_context_array();
}

#[test_case]
fn it_allocate_command_ring() {
    execute_reset_host_controller();
    execute_set_device_context_max_slot();
    CommandRingControlRegister::new(command_ring_control_register_offset())
        .unwrap()
        .setup_command_ring(&mut MikanOSPciMemoryAllocator::new())
        .unwrap();
}

#[test_case]
fn it_xhci_host_controller_initialize() {
    registers()
        .init(&mut MikanOSPciMemoryAllocator::new())
        .unwrap();
}

pub(crate) fn execute_reset_host_controller() {
    operational_registers().reset_host_controller();
}

fn execute_set_device_context_max_slot() {
    registers().setup_device_context_max_slots().unwrap()
}

fn execute_allocate_device_context_array() {
    registers()
        .allocate_device_context_array(&mut MikanOSPciMemoryAllocator::new())
        .unwrap();
}
