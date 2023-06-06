use crate::acpi::rsdp::RsdpAddr;
use crate::error::KernelResult;
use crate::kernel_bail;
use crate::volatile_bits::volatile_bits;
use volatile_bits::VolatileBitsReadable;

/// If this value added to all the others and casted to byte isn't equal to 0,
/// the table must be ignored.
#[volatile_bits(
type = u8,
add = 8
)]
#[derive(Debug)]
pub struct CheckSum(RsdpAddr);
