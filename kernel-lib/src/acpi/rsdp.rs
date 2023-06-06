use volatile_bits::{volatile_address, VolatileBitsReadable};

use crate::acpi::rsdp::check_sum::CheckSum;
use crate::acpi::rsdp::revision::Revision;
use crate::acpi::rsdp::signature::Signature;
use crate::acpi::rsdp::xsdt_address::XsdtAddress;
use crate::acpi::xsdt::Xsdt;
use crate::error::KernelResult;

pub mod check_sum;
pub mod revision;
pub mod signature;
mod xsdt_address;

#[volatile_address]
pub struct RsdpAddr(u64);

#[derive(Debug)]
pub struct Rsdp {
    addr: RsdpAddr,
    signature: Signature,
    check_sum: CheckSum,
    // oem_id: VolatileChars<6>,
    revision: Revision,
    // rsdt_address: u32,
    // length: u32,
    xsdt_address: XsdtAddress,
    // extended_address: u64,
    // extended_checksum: u8,
}


impl Rsdp {
    pub fn new(addr: RsdpAddr) -> KernelResult<Self> {
        Ok(Self {
            addr,
            signature: Signature::new(addr)?,
            check_sum: CheckSum::from(addr),
            revision: Revision::new(addr)?,
            xsdt_address: XsdtAddress::new(addr),
        })
    }


    pub fn xsdt(&self) -> KernelResult<Xsdt> {
        let xsdt_address = self
            .xsdt_address
            .read_volatile();

        Xsdt::new(xsdt_address)
    }
}
