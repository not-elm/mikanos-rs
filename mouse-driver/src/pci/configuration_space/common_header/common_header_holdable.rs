use crate::pci::configuration_space::common_header::class_code::ClassCode;
use crate::pci::configuration_space::common_header::header_type::HeaderType;
use crate::pci::configuration_space::common_header::sub_class::Subclass;
use crate::pci::configuration_space::common_header::vendor_id::VendorId;
use crate::pci::configuration_space::ConfigurationSpace;

pub trait CommonHeaderHoldable {
    fn device_slot(&self) -> u16 {
        convert_to_device_id(self.as_config_space().fetch_data_offset_at(0))
    }
    fn vendor_id(&self) -> VendorId {
        VendorId::new(convert_to_vendor_id(
            self.as_config_space().fetch_data_offset_at(0),
        ))
    }

    fn class_code(&self) -> ClassCode {
        let code = self.as_config_space().fetch_data_offset_at(0x8);

        ClassCode::try_from(convert_to_class_code(code)).unwrap_or(ClassCode::NoSupport)
    }
    fn sub_class(&self) -> Subclass {
        let offset_8 = self.as_config_space().fetch_data_offset_at(0x08);

        let sub_class = convert_to_sub_class(offset_8);

        Subclass::from_class_code(self.class_code(), sub_class)
    }
    fn header_type(&self) -> HeaderType {
        HeaderType::new(convert_to_header_type(
            self.as_config_space().fetch_data_offset_at(0x0C),
        ))
    }

    fn as_config_space(&self) -> &ConfigurationSpace;
}

pub(crate) fn convert_to_vendor_id(data_offset_0: u32) -> u16 {
    (data_offset_0 & 0xFF) as u16
}

pub(crate) fn convert_to_device_id(data_offset_0: u32) -> u16 {
    (data_offset_0 >> 8) as u16
}

pub(crate) fn convert_to_class_code(data_offset_8: u32) -> u8 {
    ((data_offset_8 >> 24) & 0xFF) as u8
}

pub(crate) fn convert_to_sub_class(data_offset_8: u32) -> u8 {
    ((data_offset_8 >> 16) & 0xFF) as u8
}

pub(crate) fn convert_to_header_type(data_offset_c: u32) -> u8 {
    ((data_offset_c >> 16) & 0xff) as u8
}

#[cfg(test)]
mod tests {
    use crate::pci::configuration_space::common_header::common_header_holdable::{
        convert_to_class_code, convert_to_device_id, convert_to_sub_class, convert_to_vendor_id,
    };

    #[test]
    fn it_convert_to_vendor_id() {
        assert_eq!(convert_to_vendor_id(0xFFFC), 0xFC);
    }

    #[test]
    fn it_convert_to_device_id() {
        assert_eq!(convert_to_device_id(0xFC32), 0xFC);
    }

    #[test]
    fn it_convert_to_class_code() {
        assert_eq!(convert_to_class_code((0xC << 24) | 0xABC), 0xC);
    }

    #[test]
    fn it_convert_to_sub_class() {
        assert_eq!(
            convert_to_sub_class(0b00000000_11110000_00000000_00000000),
            0b11110000
        );
    }
}
