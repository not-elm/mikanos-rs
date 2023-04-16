use spin::Mutex;
use x86_64::instructions::segmentation::{Segment, CS};
use x86_64::registers::segmentation::SS;
use x86_64::structures::gdt::{Descriptor, GlobalDescriptorTable};

use kernel_lib::interrupt::asm::cli;
use kernel_lib::segmentation::asm::{read_code_segment, read_stack_segment};

static mut GDT: Mutex<GlobalDescriptorTable> = Mutex::new(GlobalDescriptorTable::new());


pub fn init_gdt() {
    unsafe { init_gdt_unsafe() };
}

unsafe fn init_gdt_unsafe() {
    cli();
    let code_segment = GDT
        .lock()
        .add_entry(Descriptor::kernel_code_segment());
    let stack_segment = GDT
        .lock()
        .add_entry(Descriptor::kernel_data_segment());

    GDT.get_mut().load();
    CS::set_reg(code_segment);
    SS::set_reg(stack_segment);

    assert_eq!(read_code_segment(), code_segment.0);
    assert_eq!(read_stack_segment(), stack_segment.0);
}
