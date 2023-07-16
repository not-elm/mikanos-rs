use core::fmt::{Formatter, LowerHex, UpperHex};
use core::ops::{Deref, DerefMut};

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct EntryPointAddr(u64);


impl EntryPointAddr {
    #[inline]
    pub const fn new(addr: u64) -> Self {
        Self(addr)
    }
}


impl Deref for EntryPointAddr {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


impl DerefMut for EntryPointAddr {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}


impl UpperHex for EntryPointAddr {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let val = self.0;
        core::fmt::UpperHex::fmt(&val, f)
    }
}


impl LowerHex for EntryPointAddr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let val = self.0;
        core::fmt::LowerHex::fmt(&val, f)
    }
}
