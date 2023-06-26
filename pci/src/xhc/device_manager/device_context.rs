use xhci::context::{Device32Byte, DeviceHandler, SlotHandler};

#[repr(C, align(64))]
#[derive(Debug)]
pub struct DeviceContext(Device32Byte);

impl DeviceContext {
    pub fn slot(&self) -> &dyn SlotHandler {
        self.0.slot()
    }


    pub fn new() -> Self {
        Self(Device32Byte::default())
    }

    
    pub fn device_context_addr(&self) -> u64 {
        (&self.0 as *const Device32Byte) as u64
    }
}

impl Default for DeviceContext {
    fn default() -> Self {
        Self::new()
    }
}
