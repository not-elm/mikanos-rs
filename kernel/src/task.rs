use alloc::string::ToString;
use core::sync::atomic::AtomicUsize;
use core::sync::atomic::Ordering::Relaxed;

use kernel_lib::interrupt::asm::{cli, sti, sti_and_hlt};
use kernel_lib::interrupt::interrupt_message::TaskMessage;
use kernel_lib::task::GlobalTaskManger;
use kernel_lib::task::priority_level::PriorityLevel;

use crate::layers::COUNT_LAYER_KEY;
use crate::task::idle::idle;

mod idle;
pub mod task_message_iter;

pub static mut TASK_MANAGER: GlobalTaskManger = GlobalTaskManger::uninit();


trait FunAddr {
    unsafe fn addr(self) -> u64;
}


unsafe fn addr(f: extern "sysv64" fn(u64, u64)) -> u64 {
    f as *const () as u64
}

pub unsafe fn init() {
    TASK_MANAGER.init().unwrap();

    TASK_MANAGER
        .new_task(PriorityLevel::new(1), addr(window_count_task), 0x30);

    TASK_MANAGER
        .new_task(PriorityLevel::new(0), addr(idle), 0x00);
}


extern "sysv64" fn window_count_task(_id: u64, _data: u64) {
    static COUNT: AtomicUsize = AtomicUsize::new(0);
    loop {
        cli();

        let next_count = COUNT.fetch_add(1, Relaxed);

        let _ = unsafe {
            TASK_MANAGER
                .send_message_at(0, TaskMessage::count(COUNT_LAYER_KEY.to_string(), next_count))
        };

        sti();
    }
}





