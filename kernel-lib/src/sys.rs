use core::arch::asm;

use x86_64::registers::model_specific::{LStar, Msr, SFMask};
use x86_64::registers::rflags::RFlags;
use x86_64::VirtAddr;

mod serial_print;

pub fn init() {
    unsafe {
        Msr::new(0xc0000080).write(0x501);

        LStar::write(VirtAddr::new(syscall_entry as u64));

        Msr::new(0xc0000081).write((8 << 32) | ((16 | 3) << 48));

        SFMask::write(RFlags::empty());
    }
}


#[no_mangle]
#[naked]
extern "C" fn syscall_entry() {
    unsafe {
        asm!(
        "push rbp",
        "push rcx  // original RIP",
        "push r11  // original RFLAGS",
        "mov rcx, r10",
        "and eax, 0x7fffffff",
        "mov rbp, rsp",
        "and rsp, 0xfffffffffffffff0",
        "call {}",
        "mov rsp, rbp",
        "pop r11",
        "pop rcx",
        "pop rbp",
        "sysretq",
        sym serial_print::serial_println,
        options(noreturn)
        )
    }
}
