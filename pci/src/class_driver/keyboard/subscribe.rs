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


pub trait KeyboardSubscribable {
    fn subscribe(&self, prev_modifiers: &[KeyModifier], modifiers: &[KeyModifier], keycode: char);
}


impl<F> KeyboardSubscribable for F
where
    F: Fn(&[KeyModifier], &[KeyModifier], char),
{
    fn subscribe(&self, prev_modifiers: &[KeyModifier], modifiers: &[KeyModifier], keycode: char) {
        self(prev_modifiers, modifiers, keycode)
    }
}
