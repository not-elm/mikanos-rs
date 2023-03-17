use crate::configuration_space::common_header::class_code::ClassCode;
use crate::configuration_space::common_header::sub_class::Subclass::{
    Digitizer, Keyboard, Mouse, PciToPciBridge, Scanner, Usb,
};

#[derive(Debug, PartialEq, Eq)]
pub enum Subclass {
    // == Input Device ====
    Keyboard,
    Digitizer,
    Mouse,
    Scanner,

    // == Serial Bus ====
    Usb,

    // == Bride ====
    PciToPciBridge,

    NoSupport,
}

impl Subclass {
    pub(crate) fn from_class_code(class_code: ClassCode, sub_class: u8) -> Subclass {
        match class_code {
            ClassCode::InputDevice => from_input_device(sub_class),
            ClassCode::BridgeDevice => from_bridge(sub_class),
            ClassCode::SerialBus => from_serial_bus(sub_class),
            _ => Subclass::NoSupport,
        }
    }
}

fn from_bridge(sub_class: u8) -> Subclass {
    match sub_class {
        0x04 => PciToPciBridge,
        _ => Subclass::NoSupport,
    }
}

fn from_serial_bus(sub_class: u8) -> Subclass {
    match sub_class {
        0x03 => Usb,
        _ => Subclass::NoSupport,
    }
}

fn from_input_device(sub_class: u8) -> Subclass {
    match sub_class {
        0x00 => Keyboard,
        0x01 => Digitizer,
        2 => Mouse,
        3 => Scanner,
        _ => Subclass::NoSupport,
    }
}

#[cfg(test)]
mod tests {
    use crate::configuration_space::common_header::class_code::ClassCode;
    use crate::configuration_space::common_header::sub_class::Subclass;
    use crate::configuration_space::common_header::sub_class::Subclass::Mouse;

    #[test]
    fn it_get_input_device() {
        let class = ClassCode::InputDevice;
        let sub = Subclass::from_class_code(class, 0x02);
        assert_eq!(sub, Mouse);
    }
}
