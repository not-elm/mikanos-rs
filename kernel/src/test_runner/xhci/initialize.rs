use pci::xhci::registers::capability_registers::structural_parameters1::number_of_device_slots::NumberOfDeviceSlots;
use pci::xhci::registers::operational_registers::config_register::max_device_slots_enabled::MaxDeviceSlotsEnabled;
use pci::xhci::registers::operational_registers::usb_command_register::run_stop::RunStop;
use pci::xhci::set_device_context;

use crate::hcs1_offset;
use crate::test_runner::xhci::{config_register_offset, operation_registers_offset};

#[test_case]
fn it_set_device_contexts_enabled() {
    let run_stop = RunStop::new(operation_registers_offset());
    let max_slots = NumberOfDeviceSlots::new(hcs1_offset());
    let max_slots_en = MaxDeviceSlotsEnabled::new(config_register_offset());
    assert!(set_device_context(&run_stop, &max_slots, &max_slots_en).is_ok());
}
