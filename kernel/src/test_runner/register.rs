use kernel_lib::register::read::{read_cr3, read_r10, read_r11, read_r12, read_r13, read_r14, read_r15, read_r8, read_r9, read_rax, read_rbp, read_rbx, read_rcx, read_rdi, read_rdx, read_rflags, read_rsi, read_rsp_next};
use kernel_lib::register::write::write_rax;
use kernel_lib::task::TaskContext;

#[test_case]
fn it_read_cr3() {
    read_cr3();
}


#[test_case]
fn it_feed_registers() {
    let mut t = TaskContext::default();
    let rax = read_rax();
    let rbx = read_rbx();
    let rcx = read_rcx();
    let rdx = read_rdx();
    let rdi = read_rdi();
    let rsi = read_rsi();
    let rbp = read_rbp();
    let rsp = read_rsp_next();
    let r8 = read_r8();
    let r9 = read_r9();
    let r10 = read_r10();
    let r11 = read_r11();
    let r12 = read_r12();
    let r13 = read_r13();
    let r14 = read_r14();
    let r15 = read_r15();
    let cr3 = read_cr3();
    let flags = read_rflags();

    unsafe {
        write_rax(rax);
    }

    t.feed_registers();

    assert_eq!(t.rax, rax);
    assert_eq!(t.rbx, rbx);
    assert_eq!(t.rcx, rcx);
    assert_eq!(t.rdx, rdx);
    assert_eq!(t.rdi, rdi);
    assert_eq!(t.rsi, rsi);
    assert_eq!(t.rbp, rbp);
    assert_eq!(t.rsp, rsp);
    assert_eq!(t.r8, r8);
    assert_eq!(t.r9, r9);
    assert_eq!(t.r10, r10);
    assert_eq!(t.r11, r11);
    assert_eq!(t.r12, r12);
    assert_eq!(t.r13, r13);
    assert_eq!(t.r14, r14);
    assert_eq!(t.r15, r15);
    assert_eq!(t.cr3, cr3);
    assert_eq!(t.flags, flags);
}
