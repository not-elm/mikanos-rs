use enum_try_from::impl_enum_try_from_be;

use crate::pci::config_space::devices::class_code::ClassCode;
use crate::pci::config_space::devices::sub_class::Subclass::{Digitizer, Keyboard, Mouse, Scanner};

impl_enum_try_from_be! {
    #[repr(u8)]
    #[derive(Debug,PartialEq, Eq)]
    pub enum Subclass {
        Keyboard,
        Digitizer,
        Mouse,
        Scanner,
    },
    u8,
    (),
    ()
}

impl Subclass {
    pub(crate) fn try_new(class_code: ClassCode, sub_class: u8) -> Option<Subclass> {
        match class_code {
            ClassCode::InputDevice => from_input_device(sub_class),
            _ => None,
        }
    }
}

fn from_input_device(sub_class: u8) -> Option<Subclass> {
    Some(match sub_class {
        0x00 => Keyboard,
        0x01 => Digitizer,
        2 => Mouse,
        3 => Scanner,
        _ => None?,
    })
}

#[cfg(test)]
mod tests {
    use crate::pci::config_space::devices::class_code::ClassCode;
    use crate::pci::config_space::devices::sub_class::Subclass;
    use crate::pci::config_space::devices::sub_class::Subclass::{Keyboard, Mouse};

    #[test]
    fn it_get_input_device() {
        let class = ClassCode::InputDevice;
        let sub = Subclass::try_new(class, 0x02);
        assert_eq!(sub.unwrap_or(Keyboard), Mouse);
    }
}
