use crate::pci::config_space::common_header::class_code::ClassCode;
use crate::pci::config_space::common_header::common_header_holdable::CommonHeaderHoldable;
use crate::pci::config_space::common_header::sub_class::Subclass;
use crate::pci::config_space::device::device_base::DeviceBase;
use crate::pci::config_space::device::general_device::GeneralDevice;
use crate::pci::config_space::device::multiple_function_device::MultipleFunctionDevice;
use crate::pci::config_space::device::pci_bridge_device::PciBrideDevice;
use crate::pci::config_space::device_iter::device_slots::DeviceSlots;

pub mod device_base;
pub mod general_device;
pub mod multiple_function_device;
pub mod pci_bridge_device;

#[derive(Debug)]
pub enum PciDevice {
    General(GeneralDevice),
    PciToPciBridge(PciBrideDevice),
    MultipleFunction(MultipleFunctionDevice),
}

pub fn find_usb_mouse() -> Option<DeviceBase> {
    find_mouse_from_device_slots(DeviceSlots::new(0))
}

fn find_mouse_from_multiple_function_device(
    mut multiple_function_device: MultipleFunctionDevice,
) -> Option<DeviceBase> {
    multiple_function_device.find_map(|pci_device| find_mouse_from_pci(pci_device))
}

fn find_mouse_from_device_slots(mut device_slot: DeviceSlots) -> Option<DeviceBase> {
    device_slot.find_map(|pci_device| find_mouse_from_pci(pci_device))
}

fn find_mouse_from_pci(pci_device: PciDevice) -> Option<DeviceBase> {
    match pci_device {
        PciDevice::General(device) => some_if_mouse_class(&device),
        PciDevice::PciToPciBridge(bridge) => find_within_bridge(bridge),
        PciDevice::MultipleFunction(multiple_function_device) => {
            find_mouse_from_multiple_function_device(multiple_function_device)
        }
    }
}

fn find_within_bridge(bridge: PciBrideDevice) -> Option<DeviceBase> {
    if let Some(device) = some_if_mouse_class(&bridge) {
        return Some(device);
    }

    bridge
        .children()
        .find_map(|device_slots| find_mouse_from_pci(device_slots))
}

fn some_if_mouse_class(device: &impl CommonHeaderHoldable) -> Option<DeviceBase> {
    if device.class_code() == ClassCode::SerialBus && device.sub_class() == Subclass::Usb {
        return Some(device.to_device_base());
    }

    return None;
}
