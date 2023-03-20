use pci::xhci::registers::runtime_registers::interrupter_register_set::event_ring_segment_table_size::EventRingSegmentTableSize;
use pci::xhci::registers::runtime_registers::interrupter_register_set::interrupter_register_set_field::InterrupterRegisterSetField;

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
    // execute_reset_host_controller();
    // execute_set_device_context_max_slot();
    // execute_allocate_device_context_array();
    // let offset = interrupter_register_set_offset(0);
    // let erst_max =
    //     EventRingSegmentTableMax::new(StructuralParameters2Offset::new(mmio_base_addr()));
    // serial_println!("ERST {:b}", unsafe {
    //     *((offset.offset() + 0x04) as *const u32)
    // });

    // TODO EventRingSegmentSize TEST
    // InterrupterRegisterSet::new(offset)
    //     .setup_event_ring(&mut MikanOSPciMemoryAllocator::new())
    //     .unwrap();
}
