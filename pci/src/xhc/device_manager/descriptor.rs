use structs::configuration_descriptor::ConfigurationDescriptor;
use structs::endpoint_descriptor::EndpointDescriptor;
use structs::hid_descriptor::HidDescriptor;
use structs::interface_descriptor::InterfaceDescriptor;

pub mod descriptor_sequence;

pub mod hid;
pub mod structs;

#[derive(Debug)]
pub enum Descriptor {
    Configuration(ConfigurationDescriptor),
    Interface(InterfaceDescriptor),
    Endpoint(EndpointDescriptor),
    Hid(HidDescriptor),
    NotSupport,
}

impl Descriptor {
    pub fn interface(&self) -> Option<&InterfaceDescriptor> {
        if let Self::Interface(interface) = self {
            Some(interface)
        } else {
            None
        }
    }
}
