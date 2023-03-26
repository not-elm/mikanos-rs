use core::arch::{asm, global_asm};
use x86_64::structures::gdt::GlobalDescriptorTable;
use x86_64::structures::idt::DescriptorTable::Gdt;

pub(crate) unsafe fn set_ds_all(value: u16) {
    asm!(
    "mov ds, di",
    "mov es, di",
    "mov fs, di",
    "mov gs, di",
    in("di") value,
    );
}
global_asm!(
    r#"
asm_set_csss:
    mov ss, rsi
    push rbp
    mov rbp, rsp
    mov rax, .next
    push rdi
    push rax
    retfq
.next:
    mov rsp, rbp
    pop rbp
    ret
"#
);

pub(crate) unsafe fn set_csss(cs: u16, ss: u16) {
    asm_set_csss(cs, ss);
}

// global_asm!(
//     r#"
//
// asm_set_csss:
//     push rbp
//     mov rbp, rsp
//     mov ss, si
//     mov rax, .next
//     push rdi
//     push rax
//     retf
// .next:
//     mov rsp, rbp
//     pop rbp
//     ret
// "#
// );
extern "C" {
    fn asm_set_csss(cs: u16, ss: u16);
}

pub(crate) unsafe fn load_gdt(limit: u16, offset: u64) {
    asm!(
    "push rbp",
    "mov rbp, rsp",
    "sub rsp, 10",
    "mov [rsp], di",
    "mov [rsp + 2], rsi",
    "lgdt [rsp]",
    "mov rsp, rbp",
    "pop rbp",
    in("di") limit,
    in("rsi") offset,
    );
}

pub(crate) unsafe fn set_cr3(value: u64) {
    asm!(
    "mov cr3, rdi",
    "ret",
    in("rdi") value,
    );
}
