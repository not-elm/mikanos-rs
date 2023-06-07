use alloc::boxed::Box;

use kernel_lib::serial_println;

use crate::class_driver::keyboard::KeyboardDriver;
use crate::class_driver::mouse::mouse_driver_factory::MouseDriverFactory;
use crate::class_driver::ClassDriverOperate;
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

        if self.interface.is_keyboard() {
            return Some(Box::new(KeyboardDriver::new()));
        }

        None
    }


    pub fn endpoint_config(&self) -> EndpointConfig {
        EndpointConfig::new(&self.endpoint)
    }
}
