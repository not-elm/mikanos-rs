use kernel_lib::println;
use pci::error::{AllocateReason, PciError, PciResult};
use pci::xhci::allocator::memory_allocatable::MemoryAllocatable;
use pci::xhci::allocator::mikanos_pci_memory_allocator::MikanOSPciMemoryAllocator;
use pci::xhci::transfer::event::segment::Segment;
use pci::xhci::transfer::event::segment_table::SegmentTableAddr;

use crate::serial_println;
use crate::test_runner::xhci::registers::execute_reset_host_controller;

#[test_case]
fn it_allocate_segment() {
    execute_reset_host_controller();

    Segment::new(1, &mut MikanOSPciMemoryAllocator::new()).unwrap();
}

#[test_case]
fn it_allocate_segment_table() {
    execute_reset_host_controller();
    let mut allocator = MikanOSPciMemoryAllocator::new();
    unsafe {
        serial_println!(
            "{:x}",
            allocator
                .allocate_with_align(64, 64, 64 * 1024)
                .unwrap()
                .address()
                .unwrap()
        )
    }
    // let segment = Segment::new(1, &mut allocator).unwrap();
    // serial_println!(
    //     "{} {}",
    //     segment.base_addr().addr(),
    //     allocator.align_index(64)
    // );
    //
    // allocate_segment_table(&mut allocator).unwrap();
}

fn allocate_segment_table(allocator: &mut impl MemoryAllocatable) -> PciResult<SegmentTableAddr> {
    const SEGMENT_TABLE_SIZE: usize = 16;

    let segment_base_addr =
        unsafe { allocator.allocate_with_align(SEGMENT_TABLE_SIZE, 64, 64 * 1024) }
            .ok_or(PciError::FailedAllocate(AllocateReason::NotEnoughMemory))?
            .address()?;
    serial_println!("{}", segment_base_addr);

    Ok(SegmentTableAddr::new(segment_base_addr))
}
