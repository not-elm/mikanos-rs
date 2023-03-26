use pci::xhc::allocator::mikanos_pci_memory_allocator::MikanOSPciMemoryAllocator;
use pci::xhc::transfer::event::segment::Segment;

use crate::test_runner::xhci::registers::execute_reset_host_controller;

#[test_case]
fn it_allocate_segment() {
    execute_reset_host_controller();

    Segment::new(1, &mut MikanOSPciMemoryAllocator::new()).unwrap();
}
