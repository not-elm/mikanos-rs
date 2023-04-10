use kernel_lib::serial_println;
use pci::configuration_space::io::io_memory_accessible::real_memory_accessor::RealIoMemoryAccessor;
use pci::configuration_space::msi::InterruptCapabilityRegisterIter;

use crate::first_general_header;

#[test_case]
fn it_interrupt_capability_registers_has_one_or_more() {
    let io = RealIoMemoryAccessor::new();
    assert!(
        InterruptCapabilityRegisterIter::new(first_general_header(), io)
            .next()
            .is_some()
    );
}
#[test_case]
fn a() {
    let io = RealIoMemoryAccessor::new();
    InterruptCapabilityRegisterIter::new(first_general_header(), io)
        .for_each(|r| {
        serial_println!("{:?}", r);
    })
}
