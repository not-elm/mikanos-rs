use core::arch::asm;

#[derive(Debug, Default)]
pub struct TaskContext {
    rax: u64,
    rbx: u64,
    rcx: u64,
    rdx: u64,
    rdi: u64,
    rsi: u64,
    rsp: u64,
    rbp: u64,
    r8: u64,
    r9: u64,
    r10: u64,
    r11: u64,
    r12: u64,
    r13: u64,
    r14: u64,
    r15: u64,
}


macro_rules! feed_register {
    ($register: ident) => {
        paste::paste! {
            #[inline(always)]
            fn [<feed_ $register>](&mut self) {
                unsafe {
                    asm!(
                        concat!("mov rax, ", stringify!($register)),
                        out("rax") self.$register,
                        options(nostack, nomem, preserves_flags)
                    );
                }
            }
        }
    };
}

impl TaskContext {
    #[inline(always)]
    pub fn feed_registers(&mut self) {
        self.feed_rax();
        self.feed_rbx();
        self.feed_rcx();
        self.feed_rdx();
        self.feed_rdi();
        self.feed_rsi();
        self.feed_rsp();
        self.feed_rbp();
        self.feed_r8();
        self.feed_r9();
        self.feed_r10();
        self.feed_r11();
        self.feed_r12();
        self.feed_r13();
        self.feed_r14();
        self.feed_r15();
    }


    #[inline(always)]
    fn feed_rsp(&mut self) {
        unsafe {
            asm!("mov {}, [rsp+8]", out(reg) self.rsp, options(nostack, nomem, preserves_flags));
        }
    }


    feed_register!(rax);
    feed_register!(rbx);
    feed_register!(rcx);
    feed_register!(rdx);
    feed_register!(rdi);
    feed_register!(rsi);
    feed_register!(rbp);
    feed_register!(r8);
    feed_register!(r9);
    feed_register!(r10);
    feed_register!(r11);
    feed_register!(r12);
    feed_register!(r13);
    feed_register!(r14);
    feed_register!(r15);
}


#[cfg(test)]
mod tests {
    use crate::register::read::{
        read_r10, read_r11, read_r12, read_r13, read_r14, read_r15, read_r8, read_r9, read_rax,
        read_rbp, read_rbx, read_rcx, read_rdi, read_rdx, read_rsi, read_rsp_next,
    };
    use crate::register::write::write_rax;
    use crate::task::TaskContext;

    macro_rules! test_feed {
        ($register: ident) => {
            paste::paste! {
                #[test]
                fn [<it_feed_ $register>]() {
                    let mut t = TaskContext::default();
                    t.[<feed_ $register>]();
                    assert_eq!(t.$register, [<read_ $register>]());
                }
            }
        };
    }

    test_feed!(rax);
    test_feed!(rbx);
    test_feed!(rcx);
    test_feed!(rdx);
    test_feed!(rdi);
    test_feed!(rsi);
    test_feed!(rbp);


    #[test]
    fn it_feed_rsp() {
        let mut t = TaskContext::default();
        t.feed_rsp();
        assert_eq!(t.rsp, read_rsp_next());
    }


    #[test]
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
    }
}
