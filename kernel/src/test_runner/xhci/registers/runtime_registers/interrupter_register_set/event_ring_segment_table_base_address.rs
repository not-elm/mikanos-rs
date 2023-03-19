use pci::xhci::registers::runtime_registers::interrupter_register_set::event_ring_segment_table_base_address::EventRingSegmentTableBaseAddress;
use pci::xhci::registers::runtime_registers::interrupter_register_set::interrupter_register_set_field::InterrupterRegisterSetField;

use crate::test_runner::xhci::registers::execute_reset_host_controller;
use crate::test_runner::xhci::registers::runtime_registers::interrupter_register_set_offset;

#[test_case]
fn it_access_correct_event_ring_segment_table_base_address() {
    execute_reset_host_controller();

    let offset = interrupter_register_set_offset(0);

    assert_eq!(
        EventRingSegmentTableBaseAddress::new(offset).event_ring_segment_table_addr(),
        0
    )
}
