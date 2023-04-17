#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
#[repr(transparent)]
pub struct PhysicalAddress(u64);


impl PhysicalAddress {
    pub fn new(phys_addr: u64) -> Self {
        Self(phys_addr)
    }

    pub fn raw(&self) -> u64{
        self.0
    }

    pub fn add_u64(&self, phys_addr: u64) -> Option<Self>{
        Some(Self::new(self.raw().checked_add(phys_addr)?))
    }
}