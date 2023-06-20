use x86_64::instructions::hlt;

use kernel_lib::interrupt::asm::{cli, sti, sti_and_hlt};
use kernel_lib::serial_println;
use kernel_lib::task::CellTaskManger;

use crate::layers::{COUNT, LAYERS};

pub static mut TASK_MANAGER: CellTaskManger = CellTaskManger::uninit();


trait FunAddr {
    unsafe fn addr(self) -> u64;
}


unsafe fn addr(f: extern "sysv64" fn(u64, u64)) -> u64 {
    let address = f as *const () as u64;
    serial_println!("fn addr = 0x{:X}", address);
    address
}

pub unsafe fn init() {
    TASK_MANAGER.init().unwrap();

    TASK_MANAGER
        .new_task()
        .init_context(addr(window_count_task), 0x30);
    //
    TASK_MANAGER
        .new_task()
        .init_context(addr(idle), 0x30);

    TASK_MANAGER
        .new_task()
        .init_context(addr(idle), 0x50);
}


extern "sysv64" fn window_count_task(id: u64, data: u64) {
    cli();
    serial_println!("window_count_task = {} data = 0x{:X}", id, data);
    sti();

    let mut count = 0;
    loop {
        count += 1;
        cli();
        update_count(count);
        sti_and_hlt();
    }
}


fn update_count(count: usize) {
    LAYERS
        .layers_mut()
        .lock()
        .borrow_mut()
        .update_layer(COUNT, |layer| {
            let window = layer.require_count().unwrap();
            window.write_count(count);
        })
        .unwrap();
}


extern "sysv64" fn idle(id: u64, data: u64) {
    cli();
    serial_println!("Idle id = {} data = 0x{:X}", id, data);
    sti();

    loop {
        hlt();
    }
}
