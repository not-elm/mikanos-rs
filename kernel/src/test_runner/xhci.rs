use mouse_driver::pci::configuration_space::common_header::class_code::ClassCode;
use mouse_driver::pci::configuration_space::common_header::sub_class::Subclass;
use mouse_driver::pci::pci_device_searcher::PciDeviceSearcher;
use mouse_driver::xhci_lib::registers::capability_registers::CapabilityRegisters;

#[test_case]
fn it_fetch_mmio_base_addr() {
    assert_eq!(0x8_00_00_00_00, mmio_base_addr())
}

#[test_case]
fn it_capability_len_is_greeter_than_zero() {
    assert!(
        0 < CapabilityRegisters::new(mmio_base_addr())
            .cap_length
            .read_volatile()
    );
}

fn mmio_base_addr() -> usize {
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
