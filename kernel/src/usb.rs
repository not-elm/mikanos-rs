use kernel_lib::apic::LocalApicRegisters;
use kernel_lib::interrupt::interrupt_vector::InterruptVector;
use kernel_lib::io::io_memory_accessible::real_memory_accessor::RealIoMemoryAccessor;
use kernel_lib::volatile_bits::VolatileBitsReadable;
use pci::configuration_space::common_header::class_code::ClassCode;
use pci::configuration_space::common_header::sub_class::Subclass;
use pci::configuration_space::device::header_type::general_header::GeneralHeader;
use pci::configuration_space::msi::msi_capability_register::structs::message_data::delivery_mode::DeliveryMode;
use pci::configuration_space::msi::msi_capability_register::structs::message_data::trigger_mode::TriggerMode;
use pci::configuration_space::msi::InterruptCapabilityRegisterIter;
use pci::error::OldPciResult;
use pci::pci_device_searcher::PciDeviceSearcher;

pub mod mouse;
pub mod xhci;

pub fn enable_msi(general_header: GeneralHeader) -> OldPciResult {
    let io = RealIoMemoryAccessor::new();
    let bsp_local_apic_id: u8 = LocalApicRegisters::default()
        .local_apic_id()
        .read_volatile();

    for mut msi in InterruptCapabilityRegisterIter::new(general_header, io)
        .filter_map(|register| register.ok())
        .filter_map(|register| register.msi())
    {
        msi.enable(
            bsp_local_apic_id,
            TriggerMode::Level,
            InterruptVector::Xhci,
            DeliveryMode::Fixed,
            0,
        )?;
    }
    Ok(())
}


pub fn first_general_header() -> GeneralHeader {
    PciDeviceSearcher::new()
        .class_code(ClassCode::SerialBus)
        .sub_class(Subclass::Usb)
        .search()
        .unwrap()
        .cast_device()
        .expect_single()
        .unwrap()
        .expect_general()
        .unwrap()
}
