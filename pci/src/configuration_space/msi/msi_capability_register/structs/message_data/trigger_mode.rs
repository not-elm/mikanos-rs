use common_lib::nums::FlagConvertible;

use crate::configuration_space::msi::msi_capability_register::structs::message_data::trigger_mode::TriggerMode::{Edge, Level};

#[repr(u8)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum TriggerMode {
    Edge = 0,
    Level = 1,
}

impl TriggerMode {
    pub fn from_bit(bit: u8) -> Self {
        if bit.is_true() {
            Level
        } else {
            Edge
        }
    }

    pub fn is_level(&self) -> bool {
        matches!(self, Level)
    }
}
