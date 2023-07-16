use core::ffi::c_void;
use core::fmt::{LowerHex, UpperHex};

use uefi::table::boot::MemoryMapIter;

use common_lib::frame_buffer::FrameBufferConfig;
use common_lib::loader::entry_point::EntryPointAddr;

#[repr(transparent)]
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct EntryPoint(EntryPointAddr);


impl EntryPoint {
    #[inline]
    pub const fn new(entry_point_addr: EntryPointAddr) -> Self {
        Self(entry_point_addr)
    }

    pub fn execute(
        &self,
        frame_buffer_config: &FrameBufferConfig,
        memory_map: &MemoryMapIter,
        rsdp: &Option<*const c_void>,
        fat_volume: *mut u8,
    ) {
        let entry_point_ptr = *self.0 as *const ();
        let entry_point: extern "sysv64" fn(
            frame_buffer_config: &FrameBufferConfig,
            memory_map: &MemoryMapIter,
            rsdp: &Option<*const c_void>,
            fat_volume: *mut u8,
        ) -> () = unsafe { core::mem::transmute(entry_point_ptr) };

        entry_point(frame_buffer_config, memory_map, rsdp, fat_volume);
    }
}


impl LowerHex for EntryPoint {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::LowerHex::fmt(&self.0, f)
    }
}


impl UpperHex for EntryPoint {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::UpperHex::fmt(&self.0, f)
    }
}
