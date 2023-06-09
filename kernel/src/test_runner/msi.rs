use kernel_lib::io::io_memory_accessible::real_memory_accessor::RealIoMemoryAccessor;
use pci::configuration_space::common_header::class_code::ClassCode;
use pci::configuration_space::common_header::sub_class::Subclass;
use pci::configuration_space::device::header_type::general_header::GeneralHeader;
use pci::configuration_space::msi::InterruptCapabilityRegisterIter;
use pci::pci_device_searcher::PciDeviceSearcher;

#[test_case]
fn it_interrupt_capability_registers_has_one_or_more() {
    let io = RealIoMemoryAccessor::new();
    assert!(
        InterruptCapabilityRegisterIter::new(first_general_header(), io)
            .next()
            .is_some()
    );
}

fn first_general_header() -> GeneralHeader {
    PciDeviceSearcher::new()
        .class_code(ClassCode::SerialBus)
        .sub_class(Subclass::Usb)
        .searches()
        .unwrap()[0]
        .clone()
        .cast_device()
        .expect_single()
        .unwrap()
        .expect_general()
        .unwrap()
}
