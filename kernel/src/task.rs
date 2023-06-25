use alloc::string::ToString;
use core::sync::atomic::AtomicUsize;
use core::sync::atomic::Ordering::Relaxed;

use kernel_lib::interrupt::asm::{cli, sti, sti_and_hlt};
use kernel_lib::interrupt::interrupt_message::TaskMessage;
use kernel_lib::serial_println;
use kernel_lib::task::priority_level::PriorityLevel;

use crate::interrupt::timer::TASK_MANAGER;
use crate::layers::{COUNT_LAYER_KEY, LAYERS};
use crate::task::idle::idle;

mod idle;
pub mod task_message_iter;


trait FunAddr {
    unsafe fn addr(self) -> u64;
}


unsafe fn addr(f: extern "sysv64" fn(u64, u64)) -> u64 {
    f as *const () as u64
}

pub unsafe fn init() {
    TASK_MANAGER.init();

    TASK_MANAGER.new_task(PriorityLevel::new(1), addr(window_count_task), 0x30);

    TASK_MANAGER.new_task(PriorityLevel::new(0), addr(idle), 0x00);
}


extern "sysv64" fn window_count_task(_id: u64, _data: u64) {
    let mut count: usize = 0;
    loop {
        count += 1;
        cli();
        unsafe {
            TASK_MANAGER
                .send_message_at(
                    0,
                    TaskMessage::Count {
                        count,
                        layer_key: COUNT_LAYER_KEY.to_string(),
                    },
                )
                .unwrap();
        }
        sti_and_hlt();
    }
}
