use crate::xhc::registers::operational_registers::config_register::ConfigRegisterOffset;
use macros::VolatileBits;

/// MaxSlotsEn
///
/// 接続できるデバイスの数を設定するためのフィールド
///
/// 最大数はCapabilityRegister/hcsParams1/MaxSlotsを読み取る必要がある
///
/// Note: UsbCommandRegister/RunStopがTrueの場合、ソフトウエア側の設定は無視される
#[derive(VolatileBits)]
#[volatile_type(u8)]
pub struct MaxDeviceSlotsEnabled(usize);

impl MaxDeviceSlotsEnabled {
    pub fn new(offset: ConfigRegisterOffset) -> Self {
        Self::new_uncheck(offset.offset())
    }
}
