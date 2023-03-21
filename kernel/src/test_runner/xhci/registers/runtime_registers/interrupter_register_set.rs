use pci::xhci::allocator::mikanos_pci_memory_allocator::MikanOSPciMemoryAllocator;
use pci::xhci::registers::capability_registers::structural_parameters2::event_ring_segment_table_max::EventRingSegmentTableMax;
use pci::xhci::registers::capability_registers::structural_parameters2::structural_parameters2_field::StructuralParameters2Field;
use pci::xhci::registers::runtime_registers::interrupter_register_set::InterrupterRegisterSet;

use crate::test_runner::xhci::hcs_params2_offset;
use crate::test_runner::xhci::registers::execute_reset_host_controller;
use crate::test_runner::xhci::registers::runtime_registers::interrupter_register_set_offset;

mod event_ring_segment_size;
mod event_ring_segment_table_base_address;
mod interrupt_enable;
mod interrupt_pending;

#[test_case]
fn it_set_up_event_ring() {
    execute_reset_host_controller();

    let interrupter_register_set_offset = interrupter_register_set_offset(0);
    InterrupterRegisterSet::new(interrupter_register_set_offset)
        .setup_event_ring(
            1,
            &EventRingSegmentTableMax::new(hcs_params2_offset()),
            &mut MikanOSPciMemoryAllocator::new(),
        )
        .unwrap();
}
