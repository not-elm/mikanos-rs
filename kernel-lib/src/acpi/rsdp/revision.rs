use volatile_bits::{volatile_bits, VolatileBitsReadable};

use crate::acpi::rsdp::RsdpAddr;
use crate::error::KernelResult;
use crate::kernel_bail;

/// The revision of the ACPI
#[volatile_bits(
type = u8,
add = 15
)]
#[derive(Debug)]
pub struct Revision(RsdpAddr);


impl Revision {
    pub fn new(addr: RsdpAddr) -> KernelResult<Self> {
        let revision = Revision::from(addr);
        if revision.read_volatile() != 2 {
            return kernel_bail!("Acpi revision must be 2");
        }

        Ok(revision)
    }
}
