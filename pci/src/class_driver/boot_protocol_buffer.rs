/// ブートプロトコルに対応したHIDデバイスのデータバッファを表します。
///
/// [HID](https://www.usb.org/sites/default/files/documents/hid1_11.pdf)
#[derive(Debug)]
#[repr(transparent)]
pub struct BootProtocolBuffer<'buff>(&'buff [i8]);

impl<'buff> BootProtocolBuffer<'buff> {
    pub fn new(data_buff: &'buff [i8]) -> Self {
        Self(data_buff)
    }
    
    pub fn buff(&self) -> &[i8]{
        self.0
    }
}
