use core::arch::asm;

#[derive(Debug, Copy, Clone)]
#[repr(align(16))]
pub struct FxSaveArea([u8; 512]);


impl FxSaveArea {
    #[inline(always)]
    pub const fn new() -> Self {
        Self([0; 512])
    }


    #[inline(always)]
    pub fn buff(&self) -> &[u8] {
        &self.0
    }


    #[inline(always)]
    pub fn buff_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }


    #[inline(always)]
    pub const fn as_ptr(&self) -> *const u8 {
        self.0.as_ptr()
    }


    #[inline(always)]
    pub fn as_mut_ptr(&mut self) -> *mut u8 {
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
    pub fx_save_area: FxSaveArea,
}


macro_rules! store_register {
    ($register: ident) => {
        paste::paste! {
            #[inline(always)]
            fn [<store_ $register>](&mut self) {
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


macro_rules! store_register_32bits {
    ($register: ident) => {
        paste::paste! {
            #[inline(always)]
            fn [<store_ $register>](&mut self) {
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


macro_rules! restore_register {
    ($register: ident) => {
        paste::paste! {
            #[inline(always)]
            pub fn [<restore_ $register>](&self) {
                unsafe {
                    $crate::register::write::[<write_ $register>](self.$register);
                }
            }
        }
    };
}


impl TaskContext {
    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            rax: 0,
            rbx: 0,
            rcx: 0,
            rdx: 0,
            rdi: 0,
            rsi: 0,
            rsp: 0,
            rbp: 0,
            r8: 0,
            r9: 0,
            r10: 0,
            r11: 0,
            r12: 0,
            r13: 0,
            r14: 0,
            r15: 0,
            cr3: 0,
            rip: 0,
            flags: 0,
            cs: 0,
            ss: 0,
            fs: 0,
            gs: 0,
            fx_save_area: FxSaveArea::new(),
        }
    }


    pub fn switch_to(&mut self, next_task: &TaskContext) {
        self.store_registers();
        next_task.restore_registers();
    }


    #[inline(always)]
    pub fn store_registers(&mut self) {
        self.store_rax();
        self.store_rbx();
        self.store_rcx();
        self.store_rdx();
        self.store_rdi();
        self.store_rsi();
        self.store_rsp();
        self.store_rbp();
        self.store_r8();
        self.store_r9();
        self.store_r10();
        self.store_r11();
        self.store_r12();
        self.store_r13();
        self.store_r14();
        self.store_r15();
        self.store_cr3();
        self.store_rip();
        self.store_flags();
        self.store_cs();
        self.store_ss();
        self.store_fs();
        self.store_gs();
        self.store_fx_save_area();
    }


    #[inline(always)]
    pub fn restore_registers(&self) {
        self.fxrstor();
        self.restore_rax();
        self.restore_rbx();
        self.restore_rcx();
        self.restore_rdx();
        self.restore_rsi();
        self.restore_rbp();
        self.restore_r8();
        self.restore_r9();
        self.restore_r10();
        self.restore_r11();
        self.restore_r12();
        self.restore_r13();
        self.restore_r14();
        self.restore_r15();
        self.restore_rdi();
        unsafe { asm!("iretq") };
    }


    #[inline(always)]
    fn fxrstor(&self) {
        unsafe {
            asm!(
            "push {0}",
            "push {1}",
            "push {2}",
            "push {3}",
            "push {4}",
            "fxrstor [{5}]",
            in(reg) self.ss,
            in(reg) self.rsp,
            in(reg) self.flags,
            in(reg) self.cs,
            in(reg) self.rip,
            in(reg) self.fx_save_area.as_ptr() as u64,
            )
        }
    }


    #[inline(always)]
    fn store_rsp(&mut self) {
        unsafe {
            asm!(
            "lea rax, [rsp+8]", out("rax") self.rsp, options(nostack, nomem, preserves_flags));
        }
    }


    #[inline(always)]
    fn store_rip(&mut self) {
        unsafe {
            asm!("mov rax, [rsp]", out("rax") self.rip, options(nostack, nomem, preserves_flags));
        }
    }


    #[inline(always)]
    fn store_flags(&mut self) {
        unsafe {
            asm!(
            "pushfq",
            "pop {}",
            out(reg) self.flags,
            options(nomem, preserves_flags));
        }
    }


    #[inline(always)]
    fn store_fx_save_area(&mut self) {
        unsafe {
            asm!(
            "fxsave [rax]",
            in("rax") self.fx_save_area.as_mut_ptr(),
            options(nostack, nomem, preserves_flags));
        }
    }


    store_register!(rax);
    store_register!(rbx);
    store_register!(rcx);
    store_register!(rdx);
    store_register!(rdi);
    store_register!(rsi);
    store_register!(rbp);
    store_register!(r8);
    store_register!(r9);
    store_register!(r10);
    store_register!(r11);
    store_register!(r12);
    store_register!(r13);
    store_register!(r14);
    store_register!(r15);
    store_register!(cr3);
    store_register_32bits!(cs);
    store_register_32bits!(ss);
    store_register_32bits!(fs);
    store_register_32bits!(gs);


    restore_register!(rax);
    restore_register!(rbx);
    restore_register!(rcx);
    restore_register!(rdx);
    restore_register!(rsi);
    restore_register!(rbp);
    restore_register!(r8);
    restore_register!(r9);
    restore_register!(r10);
    restore_register!(r11);
    restore_register!(r12);
    restore_register!(r13);
    restore_register!(r14);
    restore_register!(r15);
    restore_register!(rdi);
}


#[cfg(test)]
mod tests {
    use core::arch::asm;

    use crate::register::read::{read_cs, read_fs, read_gs, read_rflags, read_rsp_next, read_ss};
    use crate::task::TaskContext;

    macro_rules! test_store {
        ($register: ident) => {
            paste::paste! {
                #[test]
                fn [<it_store_ $register>]() {
                    let mut t = TaskContext::default();
                    let v = $crate::register::read::[<read_ $register>]();
                    t.[<store_ $register>]();
                    assert_eq!(t.$register, v);
                }
            }
        };
    }

    test_store!(rax);
    test_store!(rbx);
    test_store!(rcx);
    test_store!(rdx);
    test_store!(rdi);
    test_store!(rsi);
    test_store!(rbp);


    #[test]
    fn it_store_rsp() {
        let mut t = TaskContext::default();
        let rsp = read_rsp_next();
        t.store_rsp();
        assert_eq!(t.rsp, rsp);
    }


    #[test]
    fn it_store_rip() {
        let mut t = TaskContext::default();
        let rip = rip();
        t.store_rip();
        assert_eq!(t.rip, rip);
    }


    #[test]
    fn it_store_flags() {
        let mut t = TaskContext::default();
        let flags = read_rflags();
        t.store_flags();
        assert_eq!(t.flags, flags);
    }


    #[test]
    fn it_store_cs() {
        let mut t = TaskContext::default();
        let cs = read_cs();
        t.store_cs();
        assert_eq!(t.cs, cs);
    }


    #[test]
    fn it_store_ss() {
        let mut t = TaskContext::default();
        let ss = read_ss();
        t.store_ss();
        assert_eq!(t.ss, ss);
    }


    #[test]
    fn it_store_fs() {
        let mut t = TaskContext::default();
        let fs = read_fs();
        t.store_fs();
        assert_eq!(t.fs, fs);
    }


    #[test]
    fn it_store_gs() {
        let mut t = TaskContext::default();
        let gs = read_gs();
        t.store_gs();
        assert_eq!(t.gs, gs);
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
