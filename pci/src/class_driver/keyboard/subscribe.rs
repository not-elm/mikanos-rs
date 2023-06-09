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


pub trait KeyboardSubscribable {
    fn subscribe(
        &self,
        prev_modifiers: &[KeyModifier],
        modifiers: &[KeyModifier],
        prev_keycodes: &[char],
        keycodes: &[char],
    );
}


impl<F> KeyboardSubscribable for F
    where
        F: Fn(&[KeyModifier], &[KeyModifier], &[char], &[char]),
{
    fn subscribe(
        &self,
        prev_modifiers: &[KeyModifier],
        modifiers: &[KeyModifier],
        prev_keycodes: &[char],
        keycodes: &[char],
    ) {
        self(prev_modifiers, modifiers, prev_keycodes, keycodes)
    }
}