use volatile_bits::volatile::Builder;
use volatile_bits::VolatileBitsReadable;

use crate::acpi::rsdp::RsdpAddr;

#[derive(Debug, Copy, Clone)]
pub struct XsdtAddress(RsdpAddr);


impl XsdtAddress {
    pub const fn new(addr: RsdpAddr) -> Self {
        Self(addr)
    }
}

impl VolatileBitsReadable<u64> for XsdtAddress {
    fn read_volatile(&self) -> u64 {
        let low = Builder::new(self.0)
            .add_addr(24)
            .build_readonly_type_as::<u32>()
            .read_volatile() as u64;

        let high = Builder::new(self.0)
            .add_addr(28)
            .build_readonly_type_as::<u32>()
            .read_volatile() as u64;

        (high << 32) | low
    }
}
