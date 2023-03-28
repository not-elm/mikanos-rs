pub mod command_ring;
pub mod device_context;
pub mod event;
pub mod ring;
pub(crate) mod trb_raw_data;

pub(crate) fn trb_byte_size() -> u64 {
    core::mem::size_of::<u128>() as u64
}

pub(crate) fn trb_buffer_from_address(trb_pointer: &mut u128) -> &mut [u32] {
    let ptr = trb_pointer as *mut u128;
    let raw_data = ptr.cast::<u32>();
    unsafe { core::slice::from_raw_parts_mut(raw_data, 4) }
}
