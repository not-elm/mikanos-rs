use alloc::boxed::Box;

use kernel_lib::serial_println;

use crate::class_driver::{ClassDriverOperate, keyboard};
use crate::class_driver::keyboard::subscribe::KeyModifier;
use crate::class_driver::mouse::mouse_driver_factory::MouseDriverFactory;
use crate::xhc::device_manager::descriptor::structs::endpoint_descriptor::EndpointDescriptor;
use crate::xhc::device_manager::descriptor::structs::interface_descriptor::InterfaceDescriptor;
use crate::xhc::device_manager::endpoint_config::EndpointConfig;

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

    pub fn class_driver(
        &self,
        mouse_driver_factory: &MouseDriverFactory,
    ) -> Option<Box<dyn ClassDriverOperate>> {
        serial_println!("{:?}", self.interface);

        if self.interface.is_mouse() {
            return Some(mouse_driver_factory.fact());
        }

        //TODO
        if self.interface.is_keyboard() {
            let driver = keyboard::builder::Builder::new()
                .auto_upper_if_shift()
                .build(subscribe);
            return Some(Box::new(driver));
        }

        None
    }


    pub fn endpoint_config(&self) -> EndpointConfig {
        EndpointConfig::new(&self.endpoint)
    }
}


fn subscribe(m1: &[KeyModifier], m2: &[KeyModifier], k1: &[char], k2: &[char]) {
    serial_println!("{:?}", k2);
}

