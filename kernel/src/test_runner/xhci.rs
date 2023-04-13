use pci::xhc::registers::internal::capability_registers::capability_length::CapabilityLength;
use pci::xhc::registers::internal::capability_registers::structural_parameters1::StructuralParameters1Offset;
use pci::xhc::registers::internal::capability_registers::structural_parameters2::StructuralParameters2Offset;
use pci::xhc::registers::internal::Internal;
use pci::xhc::registers::internal::memory_mapped_addr::MemoryMappedAddr;
use pci::xhc::registers::internal::operational_registers::command_ring_control_register::CommandRingControlRegisterOffset;
use pci::xhc::registers::internal::operational_registers::config_register::ConfigRegisterOffset;
use pci::xhc::registers::internal::operational_registers::device_context_base_address_array_pointer::DeviceContextBaseAddressArrayPointerOffset;
use pci::xhc::registers::internal::operational_registers::operation_registers_offset::OperationalRegistersOffset;
use pci::xhc::registers::internal::operational_registers::OperationalRegisters;
use pci::xhc::registers::internal::operational_registers::usb_status_register::usb_status_register_offset::UsbStatusRegisterOffset;

use crate::first_general_header;

mod registers;

pub(crate) fn mmio_base_addr() -> MemoryMappedAddr {
    let mouse = first_general_header();
    mouse.mmio_base_addr()
}

pub(crate) fn registers() -> Internal {
    Internal::new(mmio_base_addr()).unwrap()
}

pub(crate) fn operational_registers() -> OperationalRegisters {
    OperationalRegisters::new(operation_registers_offset()).unwrap()
}

#[allow(dead_code)]
pub(crate) fn hcs_params1_offset() -> StructuralParameters1Offset {
    StructuralParameters1Offset::new(mmio_base_addr())
}

#[allow(dead_code)]
pub(crate) fn hcs_params2_offset() -> StructuralParameters2Offset {
    StructuralParameters2Offset::new(mmio_base_addr())
}

#[allow(dead_code)]
pub(crate) fn operation_registers_offset() -> OperationalRegistersOffset {
    let mmio_base_addr = mmio_base_addr();
    let cap_length = CapabilityLength::new_check_length(mmio_base_addr).unwrap();
    OperationalRegistersOffset::new(mmio_base_addr, &cap_length)
}

#[allow(dead_code)]
pub(crate) fn config_register_offset() -> ConfigRegisterOffset {
    ConfigRegisterOffset::new(operation_registers_offset())
}

#[allow(dead_code)]
pub(crate) fn usb_status_register_offset() -> UsbStatusRegisterOffset {
    let mmio_addr = mmio_base_addr();
    let cap_length = CapabilityLength::new_check_length(mmio_addr).unwrap();
    UsbStatusRegisterOffset::new(OperationalRegistersOffset::new(mmio_addr, &cap_length))
}

#[allow(dead_code)]
pub(crate) fn dcbaap_offset() -> DeviceContextBaseAddressArrayPointerOffset {
    DeviceContextBaseAddressArrayPointerOffset::new(operation_registers_offset())
}

pub(crate) fn command_ring_control_register_offset() -> CommandRingControlRegisterOffset {
    CommandRingControlRegisterOffset::new(operation_registers_offset())
}
