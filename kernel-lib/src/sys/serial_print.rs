use alloc::ffi::CString;
use core::ffi::c_char;

use crate::serial_println;

pub extern "sysv64" fn serial_println(str: *mut c_char) {
    unsafe {
        serial_println!(
            "{}",
            CString::from_raw(str)
                .into_string()
                .unwrap()
        );
    }
}
