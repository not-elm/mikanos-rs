#![feature(pointer_byte_offsets)]
#![no_main]
#![no_std]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "sysv64" fn kernel_main(frame_buffer_base_addr: u64, frame_buffer_size: usize) -> () {
    unsafe {
        fill_display_with_white(frame_buffer_base_addr, frame_buffer_size);
    };
    common_lib::assembly::hlt_forever();
}

unsafe fn fill_display_with_white(frame_buffer_base_addr: u64, frame_buffer_size: usize) {
    let frame_buffer = frame_buffer_base_addr as *mut u8;
    let frame_buffer = core::slice::from_raw_parts_mut(frame_buffer, frame_buffer_size);
    for i in 0..frame_buffer_size {
        frame_buffer[i] = 0xFF;
    }
}

/// この関数はパニック時に呼ばれる
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
