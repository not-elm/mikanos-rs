use core::fmt::{Debug, Formatter};

use crate::xhc::registers::operational_registers::operation_registers_offset::OperationalRegistersOffset;
use crate::xhc::registers::port_registers::port::current_connect_status::CurrentConnectStatus;
use crate::xhc::registers::port_registers::port::port_enabled::PortEnabled;
use crate::xhc::registers::port_registers::port::port_register_field::PortRegisterField;
use crate::xhc::registers::port_registers::port::port_reset::PortReset;
use crate::VolatileAccessible;

mod current_connect_status;
mod port_enabled;
mod port_register_field;
mod port_reset;

///
///
///
/// [Xhci Document]: 406 Page
///
/// [Xhci Document] :
pub struct Port {
    ccs: CurrentConnectStatus,
    ped: PortEnabled,
    pr: PortReset,
}

impl Port {
    pub fn new(addr: PortRegisterAddr) -> Self {
        Self {
            ccs: CurrentConnectStatus::new(addr),
            ped: PortEnabled::new(addr),
            pr: PortReset::new(addr),
        }
    }

    pub fn is_connect(&self) -> bool {
        self.ccs.read_flag_volatile()
    }
    pub fn is_enabled_port(&self) -> bool {
        self.ped.read_flag_volatile()
    }

    pub fn reset(&self) {
        self.pr.write_flag_volatile(true);
        self.ped.write_flag_volatile(true);
        while self.pr.read_flag_volatile() {}
    }
}

impl Debug for Port {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("PortRegisters")
            .field("is_connect", &self.is_connect())
            .field("is_enabled", &self.is_enabled_port())
            .finish()
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct PortRegisterAddr(usize);

impl PortRegisterAddr {
    pub fn new(offset: OperationalRegistersOffset, port_index: u32) -> Self {
        Self(offset.offset() + 0x400 + (0x10 * port_index as usize))
    }

    pub fn addr(&self) -> usize {
        self.0
    }
}
