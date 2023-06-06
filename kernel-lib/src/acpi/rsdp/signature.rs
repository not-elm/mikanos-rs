use volatile_bits::VolatileAddress;

use crate::acpi::rsdp::RsdpAddr;
use crate::acpi::volatile_chars::CharBuff;
use crate::error::KernelResult;

#[derive(Debug)]
#[repr(transparent)]
pub struct Signature(CharBuff<8>);


impl Signature {
    pub fn new(addr: RsdpAddr) -> KernelResult<Self> {
        Ok(Self(CharBuff::<8>::new_with_check(
            addr.address(),
            "RSD PTR ",
        )?))
    }
}
