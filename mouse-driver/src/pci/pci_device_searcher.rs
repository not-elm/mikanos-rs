use crate::pci::configuration_space::common_header::class_code::ClassCode;
use crate::pci::configuration_space::common_header::common_header_holdable::CommonHeaderHoldable;
use crate::pci::configuration_space::common_header::sub_class::Subclass;
use crate::pci::configuration_space::common_header::vendor_id::VendorId;
use crate::pci::configuration_space::device::device_slots::DeviceSlots;
use crate::pci::configuration_space::device::function::multiple_function_device::MultipleFunctionDevice;
use crate::pci::configuration_space::device::function::single_function_device::SingleFunctionDevice;
use crate::pci::configuration_space::device::function::Function;
use crate::pci::configuration_space::device::header_type::pci_to_pci_bride_header::PciToPciBridgeHeader;
use crate::pci::configuration_space::ConfigurationSpace;

pub struct PciDeviceSearcher {
    vendor_id: Option<VendorId>,
    device_slot: Option<u16>,
    sub_class: Option<Subclass>,
    class_code: Option<ClassCode>,
}

impl PciDeviceSearcher {
    pub fn new() -> Self {
        Self {
            vendor_id: None,
            device_slot: None,
            sub_class: None,
            class_code: None,
        }
    }

    pub fn vendor_id(mut self, vendor_id: VendorId) -> Self {
        self.vendor_id = Some(vendor_id);
        self
    }

    pub fn device_slot(mut self, device_slot: u16) -> Self {
        self.device_slot = Some(device_slot);
        self
    }

    pub fn sub_class(mut self, sub_class: Subclass) -> Self {
        self.sub_class = Some(sub_class);
        self
    }

    pub fn class_code(mut self, class_code: ClassCode) -> Self {
        self.class_code = Some(class_code);
        self
    }

    pub fn search(&self) -> Option<ConfigurationSpace> {
        find_pci_device_with(self)
    }
}

fn find_pci_device_with(target: &PciDeviceSearcher) -> Option<ConfigurationSpace> {
    find_from_device_slots(DeviceSlots::new(0), target)
}

fn find_from_device_slots(
    mut device_slot: DeviceSlots,
    target: &PciDeviceSearcher,
) -> Option<ConfigurationSpace> {
    device_slot.find_map(|function| find_from_function(target, function))
}

fn find_from_function(
    target: &PciDeviceSearcher,
    function: Function,
) -> Option<ConfigurationSpace> {
    match function {
        Function::Single(single) => find_from_single(target, single),
        Function::Multiple(multiple) => find_from_multiple(target, multiple),
    }
}

fn find_from_single(
    target: &PciDeviceSearcher,
    single: SingleFunctionDevice,
) -> Option<ConfigurationSpace> {
    match single {
        SingleFunctionDevice::General(device) => get_if_target_device(target, &device),
        SingleFunctionDevice::PciToPciBride(bridge) => find_within_bridge(target, bridge),
    }
}

fn find_from_multiple(
    target: &PciDeviceSearcher,
    mut multiple_function_device: MultipleFunctionDevice,
) -> Option<ConfigurationSpace> {
    multiple_function_device.find_map(|function| find_from_function(target, function))
}

fn find_within_bridge(
    target: &PciDeviceSearcher,
    bridge: PciToPciBridgeHeader,
) -> Option<ConfigurationSpace> {
    if let Some(device) = get_if_target_device(target, &bridge) {
        return Some(device);
    }

    bridge
        .children()
        .find_map(|device_slots| find_from_function(target, device_slots))
}

fn get_if_target_device(
    target: &PciDeviceSearcher,
    device: &impl CommonHeaderHoldable,
) -> Option<ConfigurationSpace> {
    if let Some(vendor_id) = &target.vendor_id {
        if device.vendor_id() != *vendor_id {
            return None;
        }
    }

    if let Some(device_slot) = &target.device_slot {
        if device.device_slot() != *device_slot {
            return None;
        }
    }

    if let Some(class_code) = &target.class_code {
        if device.class_code() != *class_code {
            return None;
        }
    }
    if let Some(sub_class) = &target.sub_class {
        if device.sub_class() != *sub_class {
            return None;
        }
    }

    Some(device.as_config_space().clone())
}
