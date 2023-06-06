use crate::acpi::volatile_chars::CharBuff;
use crate::error::KernelResult;

#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
pub struct Signature(CharBuff<4>);


impl Signature {
    pub fn new_with_check(addr: u64, expected_signature: &str) -> KernelResult<Self> {
        Ok(Self(CharBuff::<4>::new_with_check(
            addr,
            expected_signature,
        )?))
    }


    pub fn new(addr: u64) -> Self {
        Self(CharBuff::<4>::new(addr))
    }


    pub fn valid(&self, sig: &str) -> bool {
        self.0.equal(sig)
    }
}
