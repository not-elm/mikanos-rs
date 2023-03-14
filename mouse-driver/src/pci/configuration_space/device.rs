use device_slots::DeviceSlots;
use function::multiple_function_device::MultipleFunctionDevice;
use header_type::pci_to_pci_bride_header::PciToPciBridgeHeader;

use crate::pci::configuration_space::common_header::class_code::ClassCode;
use crate::pci::configuration_space::common_header::common_header_holdable::CommonHeaderHoldable;
use crate::pci::configuration_space::common_header::sub_class::Subclass;
use crate::pci::configuration_space::device::function::single_function_device::SingleFunctionDevice;
use crate::pci::configuration_space::device::function::Function;
use crate::pci::configuration_space::ConfigurationSpace;

pub mod device_slots;
pub mod function;
pub mod header_type;

pub fn find_usb_mouse() -> Option<ConfigurationSpace> {
    find_mouse_from_device_slots(DeviceSlots::new(0))
}

fn find_mouse_from_multiple_function_device(
    mut multiple_function_device: MultipleFunctionDevice,
) -> Option<ConfigurationSpace> {
    multiple_function_device.find_map(|pci_device| find_mouse_from_pci(pci_device))
}

fn find_mouse_from_device_slots(mut device_slot: DeviceSlots) -> Option<ConfigurationSpace> {
    device_slot.find_map(|pci_device| find_mouse_from_pci(pci_device))
}

fn find_mouse_from_pci(pci_device: Function) -> Option<ConfigurationSpace> {
    match pci_device {
        Function::Single(single_function_device) => {
            find_mouse_from_single_function(single_function_device)
        }
        Function::Multiple(multiple_function_device) => {
            find_mouse_from_multiple_function_device(multiple_function_device)
        }
    }
}

fn find_mouse_from_single_function(
    single_function: SingleFunctionDevice,
) -> Option<ConfigurationSpace> {
    match single_function {
        SingleFunctionDevice::General(device) => some_if_mouse_class(&device),
        SingleFunctionDevice::PciToPciBride(bridge) => find_within_bridge(bridge),
    }
}

fn find_within_bridge(bridge: PciToPciBridgeHeader) -> Option<ConfigurationSpace> {
    if let Some(device) = some_if_mouse_class(&bridge) {
        return Some(device);
    }

    bridge
        .children()
        .find_map(|device_slots| find_mouse_from_pci(device_slots))
}

fn some_if_mouse_class(device: &impl CommonHeaderHoldable) -> Option<ConfigurationSpace> {
    if device.class_code() == ClassCode::SerialBus && device.sub_class() == Subclass::Usb {
        return Some(device.as_config_space().clone());
    }

    return None;
}
