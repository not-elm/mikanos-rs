use volatile_bits::VolatileBitsReadable;

use crate::error::KernelResult;
use crate::{acpi, serial_println};

mod length;
mod signature;


pub const SIZE: u64 = 4 + 4 + 1 + 1 + 6 + 8 + 4 + 4 + 4;

#[derive(Debug, Clone)]
pub struct DescriptionHeader {
    signature: signature::Signature,
    length: length::Length,
}


impl DescriptionHeader {
    pub fn new_with_check(addr: u64, expected_signature: &str) -> KernelResult<Self> {
        Ok(Self {
            signature: signature::Signature::new_with_check(addr, expected_signature)?,
            length: length::Length::from(addr),
        })
    }


    pub fn new(addr: u64) -> Self {
        Self {
            signature: signature::Signature::new(addr),
            length: length::Length::from(addr),
        }
    }


    pub fn valid_signature(&self, sig: &str) -> bool {
        self.signature.valid(sig)
    }


    pub fn count(&self) -> u64 {
        let len = self.length.read_volatile() as u64;
        (len - SIZE) / core::mem::size_of::<u64>() as u64
    }
}
