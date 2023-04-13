use kernel_lib::interrupt::IDT;

pub mod mouse;

pub fn init_idt() {
    unsafe {
        // unsafe {
        //     set_idt_entry(&mut IDT[0x40], make_idt_attr(14, 0, true, 0), addr,
        // GetCS());     LoadIDT(
        //         (core::mem::size_of::<InterruptDescriptor>() * (IDT.len() - 1)) as
        // u16,         IDT.as_ptr() as u64,
        //     );
        // }

        // IDT[InterruptVector::Xhci as usize].entry();
        IDT.load();
    }
}
