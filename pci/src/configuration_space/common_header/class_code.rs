use core::fmt::{Formatter, UpperHex};
use enum_try_from::impl_enum_try_from_be;

impl_enum_try_from_be! {
    #[repr(u8)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum ClassCode {
        MassStorage = 0x01,
        NetworkController = 0x02,
        DisplayController = 0x03,
        MultimediaDevice = 0x04,
        MemoryController = 0x05,
        BridgeDevice = 0x06,
        SimpleCommunicationControllers = 0x07,
        BaseSystemPeripherals = 0x08,
        /// マウスなど
        InputDevice = 0x09,

        SerialBus = 0x0C,
        NoSupport,
    },
    u8,
    (),
    ()
}


impl UpperHex for ClassCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:X}", (*self) as u8)
    }
}


#[cfg(test)]
mod tests {
    use crate::configuration_space::common_header::class_code::ClassCode;

    #[test]
    fn it_new_mass_controller() {
        assert_eq!(
            ClassCode::try_from(01).unwrap_or(ClassCode::NoSupport),
            ClassCode::MassStorage
        );
    }
}
