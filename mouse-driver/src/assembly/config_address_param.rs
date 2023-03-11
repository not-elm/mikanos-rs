use core::ops::Deref;

use bitfield_struct::bitfield;

/// コンフィグアドレスレジスタに書き込むためのデータ
#[bitfield(u32)]
pub struct ConfigAddrRegisterParam {
    pub register_offset: u8,
    #[bits(3)]
    pub function: usize,
    #[bits(5)]
    pub device: usize,
    pub bus: u8,
    #[bits(7)]
    _preserve: u8,
    pub enabled: bool,
}

impl Deref for ConfigAddrRegisterParam {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod test {
    use crate::assembly::config_address_param::ConfigAddrRegisterParam;

    #[test]
    fn it_new_default() {
        let p = ConfigAddrRegisterParam::new();
        assert_eq!(0, p.0);
    }

    #[test]
    fn it_deref() {
        let p = ConfigAddrRegisterParam::new().with_register_offset(1);
        assert_eq!(*p, 1);
    }
}
