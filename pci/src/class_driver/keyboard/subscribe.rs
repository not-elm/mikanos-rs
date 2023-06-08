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


pub trait KeyboardSubscribable {
    fn subscribe(&self, prev_modifiers: &[KeyModifier], modifiers: &[KeyModifier]);
}


impl<F> KeyboardSubscribable for F
where
    F: Fn(&[KeyModifier], &[KeyModifier]),
{
    fn subscribe(&self, prev_modifiers: &[KeyModifier], modifiers: &[KeyModifier]) {
        (self)(prev_modifiers, modifiers)
    }
}