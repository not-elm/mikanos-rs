use core::fmt::{Formatter, Pointer, UpperHex};
use core::ops::Deref;

#[repr(transparent)]
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct VendorId(u16);

impl VendorId {
    pub fn new(vendor_id: u16) -> Self {
        Self(vendor_id)
    }
    pub fn valid_device(&self) -> bool {
        self.0 != 0xFF
    }
}


impl UpperHex for VendorId {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:X}", self.0)
    }
}


impl Deref for VendorId {
    type Target = u16;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
