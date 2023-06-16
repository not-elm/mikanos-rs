use kernel_lib::serial_println;
use kernel_lib::task::AlignedTaskContext;

static mut TASK_A_CTX: AlignedTaskContext = AlignedTaskContext::uninit();
static mut TASK_B_CTX: AlignedTaskContext = AlignedTaskContext::uninit();

#[test_case]
#[allow(clippy::fn_to_numeric_cast)]
fn it_switch_task_context() {
    unsafe {
        let task_b_stack: [u64; 1024] = [0; 1024];
        let task_b_stack_end = task_b_stack
            .as_ptr_range()
            .end as u64;

        unsafe extern "sysv64" fn task(id: u32, data: u32) {
            serial_println!("1. Start Task B id = {} data = {}", id, data);
            TASK_B_CTX
                .switch_to(&TASK_A_CTX);
            panic!("Do not reach this code.")
        }

        TASK_B_CTX
            .update(task as u64, (task_b_stack_end & !0xF) - 8);

        TASK_A_CTX
            .switch_to(&TASK_B_CTX);
        serial_println!("2. Back to Task A");
    }
}

