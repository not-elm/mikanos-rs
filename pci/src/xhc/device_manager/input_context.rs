use crate::xhc::device_manager::device_context_index::DeviceContextIndex;
use xhci::context::{EndpointHandler, Input32Byte, InputControlHandler, InputHandler, SlotHandler};

#[repr(C, align(64))]
#[derive(Debug)]
pub struct InputContext(Input32Byte);
impl InputContext {
    pub fn new() -> Self {
        Self(Input32Byte::new_32byte())
    }

    pub fn clear_control(&mut self) {
        let control = self.control_mut();
        for i in 0..32 {
            control.clear_add_context_flag(i);
        }
    }

    pub fn copy_from_device_context(&mut self, device_context_slot: & dyn SlotHandler) {
        let device_slot_context = device_context_slot.as_ref();
        let input_slot_context = self.0.device_mut().slot_mut().as_mut();
        unsafe {
            core::ptr::copy(
                device_slot_context.as_ptr(),
                input_slot_context.as_mut_ptr(),
                device_slot_context.len(),
            );
        }
    }
    pub fn set_enable_slot_context(&mut self) {
        self.0.control_mut().set_add_context_flag(0);
    }
    pub fn set_enable_endpoint(&mut self, device_context_index: DeviceContextIndex) {
        self.0
            .control_mut()
            .set_add_context_flag(device_context_index.value());
    }
    pub fn slot_mut(&mut self) -> &mut dyn SlotHandler {
        self.0.device_mut().slot_mut()
    }
    pub fn endpoint_mut_at(&mut self, dci: usize) -> &mut dyn EndpointHandler {
        self.0.device_mut().endpoint_mut(dci)
    }
    pub fn input_context_addr(&self) -> u64 {
        (&self.0 as *const Input32Byte) as u64
    }
    fn control_mut(&mut self) -> &mut dyn InputControlHandler {
        self.0.control_mut()
    }
}

impl Default for InputContext {
    fn default() -> Self {
        Self::new()
    }
}
