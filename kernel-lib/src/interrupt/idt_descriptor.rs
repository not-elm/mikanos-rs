#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct IdtDescriptor {
    pub limit: u16,

    pub offset: u64,
}


impl IdtDescriptor {
    pub fn new(limit: u16, offset: u64) -> Self {
        Self { limit, offset }
    }
}
