use core::arch::asm;

use spin::RwLock;

use crate::register::read::read_cr3;

#[derive(Debug)]
#[repr(transparent)]
pub struct TaskContext(RwLock<AlignedTaskContext>);


// #[thread_local]
// static SWITCH_RESULT: Cell<Option<SwitchResult>> = Cell::new(None);
//
// pub struct SwitchResult {
//     prev_lock: RwLock<AlignedTaskContext>,
//     next_lock: RwLock<AlignedTaskContext>,
// }


impl TaskContext {
    #[inline(always)]
    pub const fn uninit() -> Self {
        Self(RwLock::new(AlignedTaskContext::uninit()))
    }


    pub unsafe fn update(
        &self,
        rip: u64,
        rsp: u64,
    ) {
        let mut context = self.0.write();
        context.update(rip, rsp);
    }


    // #[inline(always)]
    // pub fn switch_to(&self, next: &TaskContext) {
    //     let prev = RwLockWriteGuard::leak() as *mut AlignedTaskContext;
    //     let next = RwLockWriteGuard::leak(next.0.write()) as *mut AlignedTaskContext;
    //
    //     unsafe {
    //         asm!(
    //             "call {inner}",
    //             inner = sym asm_switch_context,
    //             in("rdi") &mut ((*next).0),
    //             in("rsi") &mut ((*prev).0),
    //             clobber_abi("sysv64")
    //         )
    //     }
    // }
}


#[derive(Debug)]
#[repr(C, align(16))]
pub struct AlignedTaskContext(TaskContextValue);

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

impl AlignedTaskContext {
    #[inline(always)]
    pub const fn uninit() -> Self {
        Self(TaskContextValue::uninit())
    }


    pub unsafe fn update(
        &mut self,
        rip: u64,
        rsp: u64,
    ) {
        self.set_rip(rip);
        self.set_rdi(1);
        self.set_rsi(42);

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
    pub fn switch_to(&mut self, next_task: &AlignedTaskContext) {
        unsafe {
            asm!(
                "call {inner}",
                inner = sym asm_switch_context,
                in("rdi") &next_task.0,
                in("rsi") &mut self.0,
                clobber_abi("sysv64")
            )
        }
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


#[derive(Debug)]
#[repr(C, packed)]
struct TaskContextValue {
    pub cr3: u64,
    pub rip: u64,
    pub flags: u64,
    _reserved1: u64,

    pub cs: u64,
    pub ss: u64,
    pub fs: u64,
    pub gs: u64,

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

    pub fx_save_area: [u8; 512],
}


impl TaskContextValue {
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


    #[inline(always)]
    #[allow(unused)]
    pub fn switch_to(&mut self, next_task: &TaskContextValue) {
        unsafe {
            asm!(
                "call {inner}",
                inner = sym asm_switch_context,
                in("rdi") next_task,
                in("rsi") self,
                clobber_abi("sysv64")
            )
        }
    }
}



macro_rules! restore_task {
    () => {
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

        "
    };
}
/// next = rdi
/// current = rsi
#[allow(unused)]
#[naked]
unsafe extern "sysv64" fn asm_switch_context(
    _next: &mut TaskContextValue,
    _current: &mut TaskContextValue,
) {
    asm!(
        "fxsave64 [rsi + 0xc0]",
        "fxrstor64 [rdi + 0xc0]",
        restore_task!(),
        "
        push QWORD PTR [rdi + 0x28]
        push QWORD PTR [rdi + 0x70]
        push QWORD PTR [rdi + 0x10]
        push QWORD PTR [rdi + 0x20]
        push QWORD PTR [rdi + 0x08]

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
