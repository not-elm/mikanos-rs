#[repr(C)]
#[repr(packed)]
pub struct InterruptDescriptorTablePointer {
    pub limit:
    pub offset: u16,
}
