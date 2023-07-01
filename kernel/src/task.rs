use alloc::string::ToString;

use kernel_lib::interrupt::asm::sti_and_hlt;
use kernel_lib::layers::LAYERS;
use kernel_lib::task::message::TaskMessage;
use kernel_lib::task::priority_level::PriorityLevel;
use kernel_lib::task::{dispatch, TASK_MANAGER};

use crate::layers::{COUNT_TEXT_LAYER2_KEY, COUNT_TEXT_LAYER_KEY};
use crate::task::idle::idle;

mod idle;
pub mod task_message_iter;


unsafe fn addr(f: extern "sysv64" fn(u64, u64)) -> u64 {
    f as *const () as u64
}


pub unsafe fn init() {
    TASK_MANAGER.init();

    TASK_MANAGER.new_task(PriorityLevel::new(1), addr(window_count_task1), 0x30);
    TASK_MANAGER.new_task(PriorityLevel::new(1), addr(window_count_task2), 0x30);

    TASK_MANAGER.new_task(PriorityLevel::new(0), addr(idle), 0x00);
}


macro_rules! count_task {
    ($num: literal, $key: expr) => {
        paste::paste! {
            extern "sysv64" fn [< window_count_task $num >](_id: u64, _data: u64) {
                let mut count: usize = 0;
                loop {
                    count += 1;
                    dispatch(move ||{
                        update_count(count, $key);
                    });
                    sti_and_hlt();
                }
            }
        }
    };
}


count_task!(1, COUNT_TEXT_LAYER_KEY);
count_task!(2, COUNT_TEXT_LAYER2_KEY);


#[inline(always)]
fn update_count(count: usize, key: &str) {
    LAYERS
        .lock()
        .update_layer(key, |layer| {
            let window = layer.require_count().unwrap();
            window.write_count(count);
        })
        .unwrap();
}
