use pci::configuration_space::common_header::class_code::ClassCode;
use pci::configuration_space::common_header::sub_class::Subclass;
use pci::pci_device_searcher::PciDeviceSearcher;
use pci::xhci::registers::capability_registers::capability_length::CapabilityLength;
use pci::xhci::registers::capability_registers::structural_parameters1::number_of_device_slots::NumberOfDeviceSlots;
use pci::xhci::registers::memory_mapped_addr::MemoryMappedAddr;
use pci::xhci::registers::operational_registers::config_register::max_device_slots_enabled::MaxDeviceSlotsEnabled;
use pci::xhci::registers::operational_registers::config_register::ConfigRegisterOffset;
use pci::xhci::registers::operational_registers::operation_registers_offset::OperationRegistersOffset;
use pci::xhci::registers::operational_registers::usb_command_register::run_stop::RunStop;
use pci::xhci::registers::operational_registers::usb_status_register::usb_status_register_offset::UsbStatusRegisterOffset;
use pci::xhci::set_device_context;

use crate::hcs1_offset;

pub mod capability_registers;
mod initialize;
mod usb_command_register;
mod usb_status_register;

pub(crate) fn mmio_base_addr() -> MemoryMappedAddr {
    let mouse = PciDeviceSearcher::new()
        .class_code(ClassCode::SerialBus)
        .sub_class(Subclass::Usb)
        .search()
        .unwrap()
        .cast_device()
        .expect_single()
        .unwrap()
        .expect_general()
        .unwrap();

    mouse.mmio_base_addr()
}

pub(crate) fn operation_registers_offset() -> OperationRegistersOffset {
    let mmio_base_addr = mmio_base_addr();
    let cap_length = CapabilityLength::new(mmio_base_addr).unwrap();
    OperationRegistersOffset::new(mmio_base_addr, cap_length)
}

pub(crate) fn config_register_offset() -> ConfigRegisterOffset {
    ConfigRegisterOffset::new(operation_registers_offset())
}

pub(crate) fn usb_status_register_offset() -> UsbStatusRegisterOffset {
    let mmio_addr = mmio_base_addr();
    let cap_length = CapabilityLength::new(mmio_addr).unwrap();
    UsbStatusRegisterOffset::new(OperationRegistersOffset::new(mmio_addr, cap_length))
}
