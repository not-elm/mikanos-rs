use crate::pci::config_space::access::config_address_register::ConfigAddrRegister;

pub trait CommonHeaderHoldable {
    fn device_id(&self) -> u16;
    fn vendor_id(&self) -> u16;
    fn status(&self) -> u16;
    fn class_code(&self) -> u16;
    fn sub_class(&self) -> u16;
    fn config_address_register(&self) -> &ConfigAddrRegister;
}

/// コンフィグデータレジスタから取得したデバイスのデータが有効なものか確認します。
/// 無効なデバイスである場合、CommonHeaderのオフセットデータは0xFFFFになります。
pub(crate) fn exists_device(data_offset_0: u32) -> bool {
    data_offset_0 != 0xFFFF
}

pub(crate) fn convert_to_vendor_id(data_offset_0: u32) -> u16 {
    (data_offset_0 & 0xFF) as u16
}

#[cfg(test)]
mod tests {
    use crate::pci::config_space::devices::common_header_loadable::convert_to_vendor_id;

    #[test]
    fn it_convert_to_vendor_id() {
        assert_eq!(convert_to_vendor_id(0xFFFC), 0xFC);
    }
}
