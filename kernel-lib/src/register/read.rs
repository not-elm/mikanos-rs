use core::arch::asm;

macro_rules! read_register {
    ($register: ident) => {
        paste::paste! {
            #[inline(always)]
            pub fn [<read_ $register>]() -> u64 {
                let r: u64;
                unsafe {
                    core::arch::asm!(
                        concat!("mov rax, ", stringify!($register)),
                        out("rax") r,
                        options(nostack, nomem, preserves_flags)
                    );
                }
                r
            }
        }
    };
}


macro_rules! read_register_32bits {
    ($register: ident) => {
        paste::paste! {
            #[inline(always)]
            pub fn [<read_ $register>]() -> u64 {
                let r: u64;
                unsafe {
                    core::arch::asm!(
                        concat!("mov ax, ", stringify!($register)),
                        out("ax") r,
                        options(nostack, nomem, preserves_flags)
                    );
                }
                r
            }
        }
    };
}


read_register!(rax);
read_register!(rsi);
read_register!(rbx);
read_register!(rcx);
read_register!(rdx);
read_register!(rdi);
read_register!(rbp);
read_register!(rsp);
read_register!(r8);
read_register!(r9);
read_register!(r10);
read_register!(r11);
read_register!(r12);
read_register!(r13);
read_register!(r14);
read_register!(r15);
read_register!(cr3);
read_register_32bits!(cs);
read_register_32bits!(ss);
read_register_32bits!(fs);
read_register_32bits!(gs);


#[inline(always)]
pub fn read_rsp_next() -> u64 {
    let r: u64;


    unsafe {
        core::arch::asm!(
        "mov rax, [rsp+8]",
        out("rax") r,
        options(nostack, nomem, preserves_flags)
        );
    }
    r
}


#[inline(always)]
pub fn read_rflags() -> u64 {
    let mut flags: u64;

    unsafe {
        asm!(
        "pushfq",
        "pop {}",
        out(reg) flags,
        options(nomem, preserves_flags));
    }
    flags
}


#[cfg(test)]
mod tests {
    use crate::register::read::{
        read_cs, read_fs, read_gs, read_rax, read_rbp, read_rbx, read_rcx, read_rdi, read_rflags,
        read_rsi, read_rsp, read_rsp_next, read_ss,
    };

    #[test]
    fn it_read_rax() {
        read_rax();
    }


    #[test]
    fn it_read_rsi() {
        read_rsi();
    }


    #[test]
    fn it_read_rbx() {
        read_rbx();
    }


    #[test]
    fn it_read_rcx() {
        read_rcx();
    }


    #[test]
    fn it_read_rdi() {
        read_rdi();
    }


    #[test]
    fn it_read_rbp() {
        read_rbp();
    }


    #[test]
    fn it_read_rsp() {
        assert_ne!(read_rsp(), 0);
    }


    #[test]
    fn it_read_rsp_next() {
        read_rsp_next();
    }


    #[test]
    fn it_read_rflags() {
        read_rflags();
    }


    #[test]
    fn it_read_cs() {
        read_cs();
    }


    #[test]
    fn it_read_ss() {
        read_ss();
    }


    #[test]
    fn it_read_fs() {
        read_fs();
    }


    #[test]
    fn it_read_gs() {
        read_gs();
    }


    macro_rules! test_r {
        ($no: literal) => {
            paste::paste! {
                #[test]
                fn [<it_read_r $no>]() {
                    $crate::register::read::[<read_r $no>]();
                }
            }
        };
    }


    test_r!(8);
    test_r!(9);
    test_r!(10);
    test_r!(11);
    test_r!(12);
    test_r!(13);
    test_r!(14);
    test_r!(15);
}
