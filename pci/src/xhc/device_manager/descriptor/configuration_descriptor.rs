use core::mem::size_of;
use core::ops::Add;
use kernel_lib::serial_println;

use crate::xhc::device_manager::descriptor::descriptor::UsbDescriptor;
use crate::xhc::device_manager::descriptor::endpoint_descriptor::{
    EndpointDescriptor, ENDPOINT_DESCRIPTOR_TYPE,
};
use crate::xhc::device_manager::descriptor::hid_descriptor::{HidDescriptor, HID_DESCRIPTOR_TYPE};
use crate::xhc::device_manager::descriptor::interface_descriptor::{
    InterfaceDescriptor, INTERFACE_DESCRIPTOR_TYPE,
};

#[repr(packed)]
#[derive(Debug, Copy, Clone)]
pub struct ConfigurationDescriptor {
    pub length: u8,
    pub descriptor_type: u8,
    pub total_length: u16,
    pub num_interfaces: u8,
    pub configuration_value: u8,
    pub configuration_id: u8,
    pub attributes: u8,
    pub max_power: u8,
}

pub struct ConfigurationDescriptors {
    descriptor_ptr: *mut u8,
    index: usize,
    len: usize,
}
pub(crate) const CONFIGURATION_DESCRIPTOR_TYPE: u8 = 2;
impl ConfigurationDescriptors {
    pub fn new(descriptor_ptr: *mut u8, len: usize) -> Self {
        Self {
            descriptor_ptr,
            index: 0,
            len,
        }
    }
}

impl Iterator for ConfigurationDescriptors {
    type Item = UsbDescriptor;

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

unsafe fn convert_to_descriptor(ptr: *mut u8) -> (usize, UsbDescriptor) {
    let descriptor_type = *ptr.add(1);
    serial_println!("TYPE = {}", descriptor_type);
    fn convert<T>(ptr: *mut u8) -> (usize, T) {
        (size_of::<T>(), unsafe { (ptr as *const T).read_volatile() })
    }

    match descriptor_type {
        CONFIGURATION_DESCRIPTOR_TYPE => {
            let (size, descriptor) = convert::<ConfigurationDescriptor>(ptr);
            (size, UsbDescriptor::Configuration(descriptor))
        }
        INTERFACE_DESCRIPTOR_TYPE => {
            let (size, descriptor) = convert::<InterfaceDescriptor>(ptr);
            (size, UsbDescriptor::Interface(descriptor))
        }
        ENDPOINT_DESCRIPTOR_TYPE => {
            let (size, descriptor) = convert::<EndpointDescriptor>(ptr);
            (size, UsbDescriptor::Endpoint(descriptor))
        }
        HID_DESCRIPTOR_TYPE => {
            let (size, descriptor) = convert::<HidDescriptor>(ptr);
            (size, UsbDescriptor::Hid(descriptor))
        }
        _ => (0, UsbDescriptor::NotSupport),
    }
}
