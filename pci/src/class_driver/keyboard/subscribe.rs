use alloc::rc::Rc;

#[derive(Debug, Copy, Clone)]
pub enum KeyModifier {
    LeftCtrl,
    LeftShift,
    LeftAlt,
    LeftGui,
    RightCtrl,
    RightShift,
    RightAlt,
    RightGui,
}

pub(crate) type BoxedKeyboardSubscriber = Rc<dyn KeyboardSubscribable>;

pub const LEFT_CONTROL: u8 = 0b000_00001;
pub const LEFT_SHIFT: u8 = 0b0000_0010;
pub const LEFT_ALT: u8 = 0b0000_0100;
pub const LEFT_GUI: u8 = 0b0000_1000;
pub const RIGHT_CONTROL: u8 = 0b0001_0000;
pub const RIGHT_SHIFT: u8 = 0b0010_0000;
pub const RIGHT_ALT: u8 = 0b0100_0000;
pub const RIGHT_GUI: u8 = 0b1000_0000;


pub trait KeyboardSubscribable {
    /// This Function is called whenever a keyboard action occurs.
    ///
    /// ## ModifierBits
    ///
    /// The modifier keys being pressed is represented by a 1 byte (8 bits).
    /// There may be multiple of those keys, and each corresponding bit is set
    /// to 1.
    ///
    /// See below for the corresponding bit for each key.
    ///
    /// - 0b0000_0001 = Left Control
    /// - 0b0000_0010 = Left Right
    /// - 0b0000_0100 = Left Alt
    /// - 0b0000_1000 = Left Gui
    /// - 0b0001_0000 = Right Control
    /// - 0b0010_0000 = Right Shift
    /// - 0b0100_0000 = Right Alt
    /// - 0b1000_0000 = Right Gui
    fn subscribe(&self, modifier_bits: u8, keycode: char);
}


impl<F> KeyboardSubscribable for F
where
    F: Fn(u8, char),
{
    fn subscribe(&self, modifier_bit: u8, keycode: char) {
        self(modifier_bit, keycode)
    }
}
