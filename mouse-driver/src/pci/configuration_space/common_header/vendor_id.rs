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

impl Deref for VendorId {
    type Target = u16;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
