use macros::VolatileFlag;

use crate::error::{PciError, PciResult};

#[derive(VolatileFlag)]
pub struct HostControllerReset(usize);
