// 割り込みベクタを表します。
#[repr(u8)]
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub enum InterruptVector {
    Xhci = 0x40,
    NotSupport,
}

impl InterruptVector {
    pub fn new(raw: u8) -> Self {
        match raw {
            0x40 => Self::Xhci,
            _ => Self::NotSupport,
        }
    }
    pub fn cast(self) -> u8 {
        self as u8
    }
}

#[cfg(test)]
mod tests {
    use crate::interrupt::interrupt_vector::InterruptVector;

    #[test]
    fn it_cast_xhci_vector_integer() {
        let xhci = InterruptVector::Xhci;
        assert_eq!(xhci.cast(), 0x40);
    }
}
