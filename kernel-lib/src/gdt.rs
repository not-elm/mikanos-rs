use core::arch::asm;

use spin::Mutex;
use x86_64::instructions::segmentation::{CS, Segment, SS};
use x86_64::instructions::tables::load_tss;
use x86_64::structures::gdt::{Descriptor, GlobalDescriptorTable};

use crate::interrupt::asm::cli;
use crate::segmentation::asm::{read_code_segment, read_stack_segment};
use crate::tss::TSS;

static mut GDT: Mutex<GlobalDescriptorTable> = Mutex::new(GlobalDescriptorTable::new());


pub fn init() {
    unsafe { init_gdt_unsafe() };
}

unsafe fn init_gdt_unsafe() {
    cli();

    let code_segment = GDT.lock().add_entry(Descriptor::kernel_code_segment());
    let stack_segment = GDT.lock().add_entry(Descriptor::kernel_data_segment());
    GDT.lock().add_entry(Descriptor::user_code_segment());
    GDT.lock().add_entry(Descriptor::user_data_segment());
    let tss_segment = GDT.lock().add_entry(Descriptor::tss_segment(TSS.get()));

    GDT.get_mut().load();
    load_tss(tss_segment);
    CS::set_reg(code_segment);
    SS::set_reg(stack_segment);

    let x: u16 = 0;
    asm!(
    "mov ds, {0:x}",
    "mov es, {0:x}",
    "mov fs, {0:x}",
    "mov gs, {0:x}",
    in(reg) x,
    options(nostack, preserves_flags)
    );

    assert_eq!(read_code_segment(), code_segment.0);
    assert_eq!(read_stack_segment(), stack_segment.0);
}
