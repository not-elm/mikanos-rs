#[inline]
pub fn read_rsp() -> u64 {
    let rsp: u64;
    unsafe {
        core::arch::asm!(
        "mov {}, [rsp]",
        out(reg) rsp,
        options(nostack, nomem, preserves_flags)
        )
    }

    rsp
}
