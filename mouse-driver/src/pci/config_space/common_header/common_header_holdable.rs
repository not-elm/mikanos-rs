use crate::pci::config_space::access::ConfigurationSpace;
use crate::pci::config_space::common_header::class_code::ClassCode;
use crate::pci::config_space::common_header::header_type::HeaderType;
use crate::pci::config_space::common_header::sub_class::Subclass;
use crate::pci::config_space::common_header::vendor_id::VendorId;
use crate::pci::config_space::device::device_base::DeviceBase;

pub trait CommonHeaderHoldable {
    fn device_id(&self) -> u16 {
        convert_to_device_id(self.config_space().fetch_data_offset_at(0))
    }
    fn vendor_id(&self) -> VendorId {
        VendorId::new(convert_to_vendor_id(
            self.config_space().fetch_data_offset_at(0),
        ))
    }

    fn class_code(&self) -> Option<ClassCode> {
        let code = self.config_space().fetch_data_offset_at(0x8);

        ClassCode::try_from(convert_to_class_code(code)).ok()
    }
    fn sub_class(&self) -> Option<Subclass> {
        let offset_8 = self.config_space().fetch_data_offset_at(0x08);

        let sub_class = convert_to_sub_class(offset_8);

        Subclass::try_new(self.class_code()?, sub_class)
    }
    fn header_type(&self) -> HeaderType {
        HeaderType::new(convert_to_header_type(
            self.config_space().fetch_data_offset_at(0x0C),
        ))
    }
    fn to_device_base(&self) -> DeviceBase {
        DeviceBase::new(self.config_space().clone())
    }
    fn config_space(&self) -> &ConfigurationSpace;
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
    use crate::pci::config_space::common_header::common_header_holdable::{
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
