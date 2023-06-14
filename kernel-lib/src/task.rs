use core::arch::asm;

#[derive(Debug, Copy, Clone)]
#[repr(align(16))]
pub struct FxSaveArea([u8; 512]);


impl FxSaveArea {
    pub fn buff(&self) -> &[u8] {
        &self.0
    }


    pub fn ptr_mut(&mut self) -> *mut u8 {
        self.0.as_mut_ptr()
    }
}


impl Default for FxSaveArea {
    fn default() -> Self {
        Self([0; 512])
    }
}


#[derive(Debug, Default)]
pub struct TaskContext {
    pub rax: u64,
    pub rbx: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rdi: u64,
    pub rsi: u64,
    pub rsp: u64,
    pub rbp: u64,
    pub r8: u64,
    pub r9: u64,
    pub r10: u64,
    pub r11: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
    pub cr3: u64,
    pub rip: u64,
    pub flags: u64,
    pub cs: u64,
    pub ss: u64,
    pub fs: u64,
    pub gs: u64,
    fx_save_area: FxSaveArea,
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


macro_rules! feed_register_32bits {
    ($register: ident) => {
        paste::paste! {
            #[inline(always)]
            fn [<feed_ $register>](&mut self) {
                unsafe {
                    asm!(
                        concat!("mov ax, ", stringify!($register)),
                        out("ax") self.$register,
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
        self.feed_cr3();
        self.feed_rip();
        self.feed_flags();
        self.feed_cs();
        self.feed_ss();
        self.feed_fs();
        self.feed_gs();
    }


    #[inline(always)]
    fn feed_rsp(&mut self) {
        unsafe {
            asm!("mov rax, [rsp+8]", out("rax") self.rsp, options(nostack, nomem, preserves_flags));
        }
    }


    #[inline(always)]
    fn feed_rip(&mut self) {
        unsafe {
            asm!("mov rax, [rsp]", out("rax") self.rip, options(nostack, nomem, preserves_flags));
        }
    }


    #[inline(always)]
    fn feed_flags(&mut self) {
        unsafe {
            asm!(
            "pushfq",
            "pop {}",
            out(reg) self.flags,
            options(nomem, preserves_flags));
        }
    }


    #[inline(always)]
    pub fn feed_fx_save_area(&mut self) {
        unsafe {
            asm!(
            "fxsave [{}]",
            in(reg) self.fx_save_area.ptr_mut(),
            options(nostack, nomem, preserves_flags));
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
    feed_register!(cr3);
    feed_register_32bits!(cs);
    feed_register_32bits!(ss);
    feed_register_32bits!(fs);
    feed_register_32bits!(gs);
}


#[cfg(test)]
mod tests {
    use core::arch::asm;

    use crate::register::read::{read_cs, read_fs, read_gs, read_rflags, read_rsp_next, read_ss};
    use crate::task::TaskContext;

    macro_rules! test_feed {
        ($register: ident) => {
            paste::paste! {
                #[test]
                fn [<it_feed_ $register>]() {
                    let mut t = TaskContext::default();
                    let v = $crate::register::read::[<read_ $register>]();
                    t.[<feed_ $register>]();
                    assert_eq!(t.$register, v);
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
        let rsp = read_rsp_next();
        t.feed_rsp();
        assert_eq!(t.rsp, rsp);
    }


    #[test]
    fn it_feed_rip() {
        let mut t = TaskContext::default();
        let rip = rip();
        t.feed_rip();
        assert_eq!(t.rip, rip);
    }


    #[test]
    fn it_feed_flags() {
        let mut t = TaskContext::default();
        let flags = read_rflags();
        t.feed_flags();
        assert_eq!(t.flags, flags);
    }


    #[test]
    fn it_feed_cs() {
        let mut t = TaskContext::default();
        let cs = read_cs();
        t.feed_cs();
        assert_eq!(t.cs, cs);
    }


    #[test]
    fn it_feed_ss() {
        let mut t = TaskContext::default();
        let ss = read_ss();
        t.feed_ss();
        assert_eq!(t.ss, ss);
    }


    #[test]
    fn it_feed_fs() {
        let mut t = TaskContext::default();
        let fs = read_fs();
        t.feed_fs();
        assert_eq!(t.fs, fs);
    }


    #[test]
    fn it_feed_gs() {
        let mut t = TaskContext::default();
        let gs = read_gs();
        t.feed_gs();
        assert_eq!(t.gs, gs);
    }


    #[test]
    fn it_feed_fx_save_area() {
        let mut t = TaskContext::default();

        t.feed_fx_save_area();
        assert!(
            !(t.fx_save_area
                .buff()
                .iter()
                .all(|b| *b == 0))
        );
    }


    #[inline(always)]
    fn rip() -> u64 {
        let rip: u64;
        unsafe {
            asm!("mov rax, [rsp]", out("rax") rip, options(nostack, nomem, preserves_flags));
        }

        rip
    }
}
