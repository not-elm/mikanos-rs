use pci::xhc::registers::capability_registers::structural_parameters2::event_ring_segment_table_max::EventRingSegmentTableMax;
use pci::xhc::registers::capability_registers::structural_parameters2::structural_parameters2_field::StructuralParameters2Field;
use pci::xhc::registers::capability_registers::structural_parameters2::StructuralParameters2Offset;
use pci::xhc::registers::runtime_registers::interrupter_register_set::event_ring_segment_table_size::EventRingSegmentTableSize;
use pci::xhc::registers::runtime_registers::interrupter_register_set::interrupter_register_set_field::InterrupterRegisterSetField;

use crate::test_runner::xhci::mmio_base_addr;
use crate::test_runner::xhci::registers::execute_reset_host_controller;
use crate::test_runner::xhci::registers::runtime_registers::interrupter_register_set_offset;

#[test_case]
fn it_access_correct_event_ring_segment_table_size() {
    execute_reset_host_controller();

    let offset = interrupter_register_set_offset(0);

    assert_eq!(
        EventRingSegmentTableSize::new(offset).event_ring_segment_table_size(),
        0
    )
}

#[test_case]
fn it_write_event_ring_segment_table_size() {
    execute_reset_host_controller();

    let offset = interrupter_register_set_offset(0);
    let erst_max =
        EventRingSegmentTableMax::new(StructuralParameters2Offset::new(mmio_base_addr()));
    EventRingSegmentTableSize::new(offset)
        .update_event_ring_segment_table_size(&erst_max, 1)
        .unwrap();
}
