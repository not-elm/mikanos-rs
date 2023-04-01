use crate::xhc::device_manager::descriptor::configuration_descriptor::ConfigurationDescriptor;
use crate::xhc::device_manager::descriptor::endpoint_descriptor::EndpointDescriptor;
use crate::xhc::device_manager::descriptor::hid_descriptor::HidDescriptor;
use crate::xhc::device_manager::descriptor::interface_descriptor::InterfaceDescriptor;

#[derive(Debug)]
pub enum UsbDescriptor {
    Configuration(ConfigurationDescriptor),
    Interface(InterfaceDescriptor),
    Endpoint(EndpointDescriptor),
    Hid(HidDescriptor),
    NotSupport,
}

impl UsbDescriptor {
    pub fn interface(&self) -> Option<&InterfaceDescriptor> {
        if let Self::Interface(interface) = self {
            Some(interface)
        } else {
            None
        }
    }
}
