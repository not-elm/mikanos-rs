use core::ffi::c_void;

use crate::acpi::rsdp::{Rsdp, RsdpAddr};
use crate::error::KernelResult;
use crate::serial_println;

mod description_header;
pub mod rsdp;
pub mod volatile_chars;
pub mod xsdt;


pub fn init_acpi_timer(rsdp: Option<*const c_void>) -> KernelResult<()> {
    if let Some(rsdp) = rsdp {
        let rsdp = Rsdp::new(RsdpAddr::from(rsdp as u64))?;
        let xsdt = rsdp.xsdt()?;
        let fadt = xsdt.fadt();
        serial_println!("{:?}", fadt);
    }

    Ok(())
}
