use core::mem::size_of;

use crate::xhc::device_manager::descriptor::structs::configuration_descriptor::{
    ConfigurationDescriptor, CONFIGURATION_DESCRIPTOR_TYPE,
};
use crate::xhc::device_manager::descriptor::structs::endpoint_descriptor::{
    EndpointDescriptor, ENDPOINT_DESCRIPTOR_TYPE,
};
use crate::xhc::device_manager::descriptor::structs::hid_descriptor::{HidDescriptor, HID_DESCRIPTOR_TYPE};
use crate::xhc::device_manager::descriptor::structs::interface_descriptor::{
    InterfaceDescriptor, INTERFACE_DESCRIPTOR_TYPE,
};
use crate::xhc::device_manager::descriptor::Descriptor;

pub struct DescriptorSequence {
    descriptor_ptr: *mut u8,
    index: usize,
    len: usize,
}

impl DescriptorSequence {
    pub fn new(descriptor_ptr: *mut u8, len: usize) -> Self {
        Self {
            descriptor_ptr,
            index: 0,
            len,
        }
    }
}

impl Iterator for DescriptorSequence {
    type Item = Descriptor;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len <= self.index {
            return None;
        }

        let ptr = unsafe { self.descriptor_ptr.add(self.index) };
        let (descriptor_size, descriptor) = unsafe { convert_to_descriptor(ptr) };
        self.index += descriptor_size;

        Some(descriptor)
    }
}

unsafe fn convert_to_descriptor(ptr: *mut u8) -> (usize, Descriptor) {
    let descriptor_type = *ptr.add(1);

    fn convert<T>(ptr: *mut u8) -> (usize, T) {
        (size_of::<T>(), unsafe { (ptr as *const T).read_volatile() })
    }

    match descriptor_type {
        CONFIGURATION_DESCRIPTOR_TYPE => {
            let (size, descriptor) = convert::<ConfigurationDescriptor>(ptr);
            (size, Descriptor::Configuration(descriptor))
        }
        INTERFACE_DESCRIPTOR_TYPE => {
            let (size, descriptor) = convert::<InterfaceDescriptor>(ptr);
            (size, Descriptor::Interface(descriptor))
        }
        ENDPOINT_DESCRIPTOR_TYPE => {
            let (size, descriptor) = convert::<EndpointDescriptor>(ptr);

            (size, Descriptor::Endpoint(descriptor))
        }
        HID_DESCRIPTOR_TYPE => {
            let (size, descriptor) = convert::<HidDescriptor>(ptr);
            (size, Descriptor::Hid(descriptor))
        }
        _ => (0, Descriptor::NotSupport),
    }
}
