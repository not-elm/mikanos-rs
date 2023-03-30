pub mod scratchpad_buffer_ptr;
pub mod scratchpad_buffers_array_ptr;

#[repr(transparent)]
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct DeviceContextArrayPtr(u64);

impl DeviceContextArrayPtr {
    pub fn new(address: u64) -> Self {
        Self(address)
    }

    pub fn set_device_context_at(&mut self, index: usize, device_context_addr: u64) {
        unsafe {
            let ptr = (self.0 as *mut u64).add(index);

            ptr.write(device_context_addr);
        }
    }
}
