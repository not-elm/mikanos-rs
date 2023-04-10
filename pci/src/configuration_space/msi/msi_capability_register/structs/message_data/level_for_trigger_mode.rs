use common_lib::nums::FlagConvertible;

use crate::configuration_space::msi::msi_capability_register::structs::message_data::level_for_trigger_mode::LevelForTriggerMode::{Assert, DeAssert};

#[repr(u8)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum LevelForTriggerMode {
    DeAssert = 0,
    Assert = 1,
}

impl LevelForTriggerMode {
    pub fn from_bit(bit: u8) -> Self {
        if bit.is_true() {
            Assert
        } else {
            DeAssert
        }
    }
}
