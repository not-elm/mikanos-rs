use crate::class_driver;
use crate::class_driver::ClassDriver;
use crate::configuration_space::common_header::sub_class::Subclass::Mouse;
use crate::xhc::device_manager::descriptor::structs::endpoint_descriptor::EndpointDescriptor;
use crate::xhc::device_manager::descriptor::structs::interface_descriptor::InterfaceDescriptor;
use crate::xhc::device_manager::endpoint_config::EndpointConfig;
use alloc::boxed::Box;

pub struct HidDeviceDescriptors {
    interface: InterfaceDescriptor,
    endpoint: EndpointDescriptor,
}

impl HidDeviceDescriptors {
    pub fn new(interface: InterfaceDescriptor, endpoint: EndpointDescriptor) -> Self {
        Self {
            interface,
            endpoint,
        }
    }

    pub fn class_driver(&self) -> Option<Box<dyn ClassDriver>> {
        if self.interface.is_mouse() {
            return Some(Box::new(class_driver::mouse::Mouse::new()));
        } else {
            None
        }
    }

    pub fn endpoint_config(&self) -> EndpointConfig {
        EndpointConfig::new(&self.endpoint)
    }
}
