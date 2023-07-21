use core::arch::asm;

use x86_64::registers::control::{Efer, EferFlags};
use x86_64::registers::model_specific::{LStar, Msr, SFMask};
use x86_64::registers::rflags::RFlags;
use x86_64::VirtAddr;

use crate::serial_println;

#[no_mangle]
pub extern "sysv64" fn d() {
    serial_println!("CALL");
}

#[no_mangle]
#[naked]
pub extern "C" fn a() {
    unsafe {
        asm!(
        "call {}",
        sym d,
        options(noreturn)
        )
    }
}

pub fn init() {
    unsafe {
        Efer::write(Efer::read() | EferFlags::SYSTEM_CALL_EXTENSIONS);

        LStar::write(VirtAddr::new(a as u64));

        Msr::new(0xc0000081)
            .write((8 << 32) | ((16 | 3) << 48));

        SFMask::write(RFlags::empty());
    }
}