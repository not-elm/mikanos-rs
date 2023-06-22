use alloc::string::ToString;

use kernel_lib::interrupt::asm::{cli, sti, sti_and_hlt};
use kernel_lib::interrupt::interrupt_message::TaskMessage;
use kernel_lib::serial_println;
use kernel_lib::task::CellTaskManger;
use kernel_lib::task::priority_level::PriorityLevel;

use crate::layers::COUNT;
use crate::task::idle::idle;

mod idle;
pub mod task_message_iter;

pub static mut TASK_MANAGER: CellTaskManger = CellTaskManger::uninit();


trait FunAddr {
    unsafe fn addr(self) -> u64;
}


unsafe fn addr(f: extern "sysv64" fn(u64, u64)) -> u64 {
    let address = f as *const () as u64;

    address
}

pub unsafe fn init() {
    TASK_MANAGER.init().unwrap();

    TASK_MANAGER
        .new_task(PriorityLevel::new(1))
        .init_context(addr(window_count_task), 0x30);

    TASK_MANAGER
        .new_task(PriorityLevel::new(0))
        .init_context(addr(idle), 0x00);
}


extern "sysv64" fn window_count_task(_id: u64, _data: u64) {
    let mut count = 0;
    loop {
        cli();
        count += 1;

        unsafe {
            TASK_MANAGER
                .send_message_at(0, TaskMessage::count(COUNT.to_string(), count))
                .unwrap();
        }

        sti_and_hlt();
    }
}





