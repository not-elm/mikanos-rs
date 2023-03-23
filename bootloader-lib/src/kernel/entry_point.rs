use core::fmt::LowerHex;
use uefi::table::boot::MemoryMapIter;

use common_lib::frame_buffer::FrameBufferConfig;

#[derive(Debug)]
pub struct EntryPoint(u64);

impl EntryPoint {
    pub fn new(entry_point_addr: u64) -> Self {
        Self(entry_point_addr)
    }
    pub fn execute(&self, frame_buffer_config: &FrameBufferConfig, memory_map: &MemoryMapIter) {
        let entry_point_ptr = self.0 as *const ();
        let entry_point: extern "sysv64" fn(
            frame_buffer_config: &FrameBufferConfig,
            memory_map: &MemoryMapIter,
        ) -> () = unsafe { core::mem::transmute(entry_point_ptr) };
        entry_point(frame_buffer_config, memory_map);
    }
}

impl LowerHex for EntryPoint {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let val = self.0;

        core::fmt::LowerHex::fmt(&val, f)
    }
}
