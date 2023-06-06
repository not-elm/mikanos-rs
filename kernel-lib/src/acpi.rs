use core::ffi::c_void;

use crate::acpi::fadt::Fadt;
use crate::acpi::rsdp::{Rsdp, RsdpAddr};
use crate::error::KernelResult;
use crate::{kernel_bail, kernel_error};

mod description_header;
pub mod fadt;
pub mod rsdp;
pub mod volatile_chars;
pub mod xsdt;


pub fn init_acpi_timer(rsdp: Option<*const c_void>) -> KernelResult<Fadt> {
    if let Some(rsdp) = rsdp {
        let rsdp = Rsdp::new(RsdpAddr::from(rsdp as u64))?;
        let xsdt = rsdp.xsdt()?;
        let fadt = xsdt
            .fadt()
            .ok_or(kernel_error!("Not Found FADT"))?;
        return Ok(fadt);
    }

    kernel_bail!("Not Found FADT")
}
