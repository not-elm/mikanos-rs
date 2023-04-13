use kernel_lib::error::KernelResult;
use kernel_lib::interrupt::gate_type::GateType;
use kernel_lib::interrupt::IDT;
use kernel_lib::interrupt::interrupt_descriptor_attribute::InterruptDescriptorAttribute;
use kernel_lib::interrupt::interrupt_vector::InterruptVector;

use self::mouse::interrupt_mouse_handler;

pub mod mouse;

pub fn init_idt() -> KernelResult {
    unsafe {
        let type_attribute = InterruptDescriptorAttribute::new()
            .with_gate_type(GateType::InterruptGate)
            .with_present(true);


        IDT[InterruptVector::Xhci].set_handler(interrupt_mouse_handler, type_attribute)?;
        IDT.load();
    }

    Ok(())
}
