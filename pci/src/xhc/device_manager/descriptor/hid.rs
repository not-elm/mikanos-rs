use alloc::boxed::Box;

use crate::class_driver::mouse::mouse_driver_factory::MouseDriverFactory;
use crate::class_driver::mouse::mouse_subscribe_driver::MouseSubscriber;
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

    pub fn class_driver<T: MouseSubscriber + Clone>(
        &self,
        mouse_driver_factory: &MouseDriverFactory<T>,
    ) -> Option<Box<dyn ClassDriverOperate>> {
        if self.interface.is_mouse() {
            return Some(mouse_driver_factory.fact());
        } else {
            None
        }
    }

    pub fn endpoint_config(&self) -> EndpointConfig {
        EndpointConfig::new(&self.endpoint)
    }
}
