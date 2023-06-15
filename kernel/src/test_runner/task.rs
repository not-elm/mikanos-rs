use common_lib::array::array_eq;
use kernel_lib::register::read::{
    read_cr3, read_r10, read_r11, read_r12, read_r13, read_r14, read_r15, read_r8, read_r9,
    read_rax, read_rbp, read_rbx, read_rcx, read_rdi, read_rdx, read_rflags, read_rsi,
    read_rsp_next,
};
use kernel_lib::register::write::write_rax;
use kernel_lib::serial_println;
use kernel_lib::task::{FxSaveArea, TaskContext};

#[test_case]
fn it_store_registers() {
    let mut fx_save_area = FxSaveArea::default();
    unsafe {
        core::arch::asm!(
        "fxsave [{}]",
        in(reg) fx_save_area.as_mut_ptr(),
        options(nostack, nomem, preserves_flags));
    }

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

    t.store_registers();

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
    assert!(array_eq(t.fx_save_area.buff(), fx_save_area.buff()));
}


macro_rules! test_restore {
    ($register: ident) => {
        paste::paste! {
            #[test_case]
            fn [<it_restore_ $register>]() {
                let mut t = TaskContext::default();
                t.$register = 0x10;
                t.[<restore_ $register>]();
                assert_eq!([<read_ $register>](), 0x10)
            }
        }
    };
}


test_restore!(rax);
test_restore!(rbx);
test_restore!(rcx);
test_restore!(rdx);
test_restore!(r8);
test_restore!(r9);
test_restore!(r10);
test_restore!(r11);
test_restore!(r12);
test_restore!(r13);
test_restore!(r14);
test_restore!(r15);


static mut TASK_A_CTX: TaskContext = TaskContext::new();
static mut TASK_B_CTX: TaskContext = TaskContext::new();

#[test_case]
#[allow(clippy::fn_to_numeric_cast)]
fn it_switch() {
    unsafe {
        let task_b_stack: [u64; 1024] = [0; 1024];

        let task_b_stack_end = task_b_stack
            .as_ptr_range()
            .end as u64;


        fn task(id: isize, data: isize) {
            serial_println!("1. Start Task B id = {} data = {}", id, data);
            unsafe {
                serial_println!("Task A = {:?}", TASK_A_CTX);
            };
            unsafe { TASK_B_CTX.switch_to(&TASK_A_CTX) };
        }

        TASK_B_CTX.rip = task as u64;
        TASK_B_CTX.rdi = 1;
        TASK_B_CTX.rsi = 42;

        TASK_B_CTX.cr3 = read_cr3();
        TASK_B_CTX.flags = 0x202;
        TASK_B_CTX.cs = 1 << 3;
        TASK_B_CTX.ss = 2 << 3;
        TASK_B_CTX.rsp = (task_b_stack_end & !0xf) - 8;
        TASK_B_CTX
            .fx_save_area
            .as_mut_ptr()
            .add(24)
            .cast::<u32>()
            .write_volatile(0x1F80);

        TASK_A_CTX.switch_to(&TASK_B_CTX);
        serial_println!("2. Back to Task A");
    }
}
