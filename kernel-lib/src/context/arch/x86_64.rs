use core::arch::asm;

use crate::control_registers::read_cr3;

// #[derive(Debug)]
// #[repr(transparent)]
// // TODO: Support Context with Arch
// pub struct TaskContext(RwLock<AlignedContext>);


// #[thread_local]
// static SWITCH_RESULT: Cell<Option<SwitchResult>> = Cell::new(None);
//
// pub struct SwitchResult {
//     prev_lock: RwLock<AlignedTaskContext>,
//     next_lock: RwLock<AlignedTaskContext>,
// }


// impl TaskContext {
//     // #[inline(always)]
//     // pub const fn uninit() -> Self {
//     //     Self(RwLock::new(TaskContext::uninit()))
//     // }
//     //
//     //
//     // pub unsafe fn update(&self, rip: u64, rsp: u64) {
//     //     let mut context = self.0.write();
//     //     context.update(rip, rsp);
//     // }
//
//
//     // #[inline(always)]
//     // pub fn switch_to(&self, next: &TaskContext) {
//     //     let prev = RwLockWriteGuard::leak() as *mut AlignedTaskContext;
//     //     let next = RwLockWriteGuard::leak(next.0.write()) as *mut
//     // AlignedTaskContext;
//     //
//     //     unsafe {
//     //         asm!(
//     //             "call {inner}",
//     //             inner = sym asm_switch_context,
//     //             in("rdi") &mut ((*next).0),
//     //             in("rsi") &mut ((*prev).0),
//     //             clobber_abi("sysv64")
//     //         )
//     //     }
//     // }
// }


#[derive(Debug, Clone, Eq, PartialEq)]
#[repr(C, align(16))]
pub struct Context(pub ContextValue);

macro_rules! property {
    ($v: ident) => {
        paste::paste! {
            pub fn $v(&self) -> u64{
                self.0.$v
            }

            pub fn [<set_$v>](&mut self, $v: u64){
                self.0.$v = $v;
            }
        }
    };
}


impl Context {
    #[inline(always)]
    pub const fn uninit() -> Self {
        Self(ContextValue::uninit())
    }


    pub unsafe fn init_context(&mut self, rip: u64, rdi: u64, rsi: u64, rsp: u64) {
        self.set_rip(rip);
        self.set_rdi(rdi);
        self.set_rsi(rsi);
        self.set_cr3(read_cr3());
        self.set_flags(0x202);
        self.set_cs(1 << 3);
        self.set_ss(2 << 3);
        self.set_rsp(rsp);
        self.0
            .fx_save_area
            .as_mut_ptr()
            .add(24)
            .cast::<u32>()
            .write_volatile(0x1F80);
    }


    #[inline(always)]
    pub fn switch_to(&self, next_task: &Context) {
        asm_switch_context(&next_task.0, &self.0);
    }

    
    property!(cr3);
    property!(rip);
    property!(flags);
    property!(cs);
    property!(ss);
    property!(fs);
    property!(gs);
    property!(rax);
    property!(rbx);
    property!(rcx);
    property!(rdx);
    property!(rdi);
    property!(rsi);
    property!(rsp);
    property!(rbp);
    property!(r8);
    property!(r9);
    property!(r10);
    property!(r11);
    property!(r12);
    property!(r13);
    property!(r14);
    property!(r15);
}


#[derive(Debug, Clone, Eq, PartialEq)]
#[repr(C, packed)]
pub struct ContextValue {
    // 0x00
    pub cr3: u64,
    // 0x08
    pub rip: u64,
    // 0x10
    pub flags: u64,
    // 0x18
    _reserved1: u64,

    // 0x20
    pub cs: u64,
    // 0x28
    pub ss: u64,
    // 0x30
    pub fs: u64,
    // 0x38
    pub gs: u64,

    // 0x40
    pub rax: u64,
    // 0x48
    pub rbx: u64,
    // 0x50
    pub rcx: u64,
    // 0x58
    pub rdx: u64,
    // 0x60
    pub rdi: u64,
    // 0x68
    pub rsi: u64,
    // 0x70
    pub rsp: u64,
    // 0x78
    pub rbp: u64,

    // 0x80
    pub r8: u64,
    // 0x88
    pub r9: u64,
    // 0x90
    pub r10: u64,
    // 0x98
    pub r11: u64,
    // 0xA0
    pub r12: u64,
    // 0xA8
    pub r13: u64,
    // 0xB0
    pub r14: u64,
    // 0xB8
    pub r15: u64,

    // 0xC0
    pub fx_save_area: [u8; 512],
}


impl ContextValue {
    #[inline(always)]
    pub const fn uninit() -> Self {
        Self {
            rax: 0,
            rbx: 0,
            rcx: 0,
            rdx: 0,
            _reserved1: 0,
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
            fx_save_area: [0; 512],
        }
    }
}


/// next = rdi
/// current = rsi
#[allow(unused)]
#[naked]
pub extern "sysv64" fn asm_switch_context(_next: &ContextValue, _current: &ContextValue) {
    unsafe {
        asm!(
        "
        mov [rsi + 0x40], rax
        mov [rsi + 0x48], rbx
        mov [rsi + 0x50], rcx
        mov [rsi + 0x58], rdx
        mov [rsi + 0x60], rdi
        mov [rsi + 0x68], rsi

        lea rax, [rsp + 8]
        mov [rsi + 0x70], rax
        mov [rsi + 0x78], rbp

        mov [rsi + 0x80], r8
        mov [rsi + 0x88], r9
        mov [rsi + 0x90], r10
        mov [rsi + 0x98], r11
        mov [rsi + 0xa0], r12
        mov [rsi + 0xa8], r13
        mov [rsi + 0xb0], r14
        mov [rsi + 0xb8], r15

        mov rax, cr3
        mov [rsi + 0x00], rax
        mov rax, [rsp]
        mov [rsi + 0x08], rax
        pushfq
        pop QWORD PTR [rsi + 0x10]

        mov ax, cs
        mov [rsi + 0x20], rax
        mov bx, ss
        mov [rsi + 0x28], rbx
        mov cx, fs
        mov [rsi + 0x30], rcx
        mov dx, gs
        mov [rsi + 0x38], rdx
        fxsave64 [rsi + 0xc0]

        // SS
        push QWORD PTR [rdi + 0x28]
        // RSP
        push QWORD PTR [rdi + 0x70]
        // RFLAGS
        push QWORD PTR [rdi + 0x10]
        // CS
        push QWORD PTR [rdi + 0x20]
        // RIP
        push QWORD PTR [rdi + 0x08]

        fxrstor64 [rdi + 0xc0]

        mov rax, [rdi + 0x00]
        mov cr3, rax
        mov rax, [rdi + 0x30]
        mov fs, ax
        mov rax, [rdi + 0x38]
        mov gs, ax

        mov rax, [rdi + 0x40]
        mov rbx, [rdi + 0x48]
        mov rcx, [rdi + 0x50]
        mov rdx, [rdi + 0x58]
        mov rsi, [rdi + 0x68]
        mov rbp, [rdi + 0x78]
        mov r8,  [rdi + 0x80]
        mov r9,  [rdi + 0x88]
        mov r10, [rdi + 0x90]
        mov r11, [rdi + 0x98]
        mov r12, [rdi + 0xa0]
        mov r13, [rdi + 0xa8]
        mov r14, [rdi + 0xb0]
        mov r15, [rdi + 0xb8]

        mov rdi, [rdi + 0x60]

        iretq
        ",
        options(noreturn)
        )
    }
}
