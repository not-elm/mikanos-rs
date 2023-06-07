use kernel_lib::interrupt::interrupt_vector::InterruptVector;

use crate::configuration_space::msi::msi_capability_register::structs::from_u32::TryFromU32;
use crate::configuration_space::msi::msi_capability_register::structs::message_data::delivery_mode::DeliveryMode;
use crate::configuration_space::msi::msi_capability_register::structs::message_data::level_for_trigger_mode::LevelForTriggerMode;
use crate::configuration_space::msi::msi_capability_register::structs::message_data::trigger_mode::TriggerMode;
use crate::error::PciResult;

pub mod delivery_mode;
pub mod level_for_trigger_mode;
pub mod trigger_mode;

#[derive(Debug, Clone)]
pub struct MessageData {
    vector: InterruptVector,
    delivery_mode: DeliveryMode,
    level_for_trigger_mode: LevelForTriggerMode,
    trigger_mode: TriggerMode,
}

impl MessageData {
    pub fn raw(&self) -> u32 {
        ((self.trigger_mode as u32) << 15)
            | ((self.level_for_trigger_mode as u32) << 14)
            | ((self.delivery_mode as u32) << 8)
            | self.vector as u32
    }


    pub fn set_vector(&mut self, vector: InterruptVector) {
        self.vector = vector;
    }


    pub fn set_delivery_mode(&mut self, delivery_mode: DeliveryMode) {
        self.delivery_mode = delivery_mode;
    }


    pub fn set_trigger_mode(&mut self, trigger_mode: TriggerMode) {
        self.trigger_mode = trigger_mode;
    }


    pub fn set_level_for_trigger_mode(&mut self, level_for_trigger_mode: LevelForTriggerMode) {
        self.level_for_trigger_mode = level_for_trigger_mode;
    }
}


impl TryFromU32<MessageData> for MessageData {
    fn try_from_u32(raw_value: u32) -> PciResult<MessageData> {
        Ok(Self {
            vector: InterruptVector::new((raw_value & 0xFF) as u8),
            delivery_mode: DeliveryMode::new(((raw_value >> 8) & 0b111) as u8)?,
            level_for_trigger_mode: LevelForTriggerMode::from_bit(((raw_value >> 14) & 0b1) as u8),
            trigger_mode: TriggerMode::from_bit(((raw_value >> 15) & 0b1) as u8),
        })
    }
}


#[cfg(test)]
mod tests {
    use kernel_lib::interrupt::interrupt_vector::InterruptVector;

    use crate::configuration_space::msi::msi_capability_register::structs::from_u32::TryFromU32;
    use crate::configuration_space::msi::msi_capability_register::structs::message_data::delivery_mode::DeliveryMode;
    use crate::configuration_space::msi::msi_capability_register::structs::message_data::MessageData;

    const RAW_DATA: u32 = 0b1000000;

    #[test]
    fn it_convert_to_message_data() {
        assert!(MessageData::try_from_u32(RAW_DATA).is_ok());
    }

    #[test]
    fn it_convert_vector() {
        assert_eq!(
            MessageData::try_from_u32(RAW_DATA)
                .unwrap()
                .vector,
            InterruptVector::Xhci
        );
    }

    #[test]
    fn it_convert_fix() {
        assert_eq!(
            MessageData::try_from_u32(RAW_DATA)
                .unwrap()
                .delivery_mode,
            DeliveryMode::Fixed
        );
    }

    #[test]
    fn it_as_raw() {
        assert_eq!(
            MessageData::try_from_u32(RAW_DATA)
                .unwrap()
                .raw(),
            RAW_DATA
        );
    }
}
