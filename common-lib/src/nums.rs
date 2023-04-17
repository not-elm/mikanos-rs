pub trait FlagConvertible {
    /// 自身をBoolean値に変換します。
    fn is_true(&self) -> bool;
    fn is_false(&self) -> bool {
        !self.is_true()
    }
}


impl FlagConvertible for usize {
    fn is_true(&self) -> bool {
        *self != 0
    }
}


impl FlagConvertible for u128 {
    fn is_true(&self) -> bool {
        *self != 0
    }
}


impl FlagConvertible for u64 {
    fn is_true(&self) -> bool {
        *self != 0
    }
}

impl FlagConvertible for u32 {
    fn is_true(&self) -> bool {
        *self != 0
    }
}

impl FlagConvertible for u16 {
    fn is_true(&self) -> bool {
        *self != 0
    }
}

impl FlagConvertible for u8 {
    fn is_true(&self) -> bool {
        *self != 0
    }
}

pub trait FlagNumConvertible<Num> {
    fn into_bit(self) -> Num;
}

impl FlagNumConvertible<u8> for bool {
    fn into_bit(self) -> u8 {
        to_bit(self) as u8
    }
}

impl FlagNumConvertible<u16> for bool {
    fn into_bit(self) -> u16 {
        to_bit(self) as u16
    }
}


impl FlagNumConvertible<u32> for bool {
    fn into_bit(self) -> u32 {
        to_bit(self) as u32
    }
}

impl FlagNumConvertible<u64> for bool {
    fn into_bit(self) -> u64 {
        to_bit(self) as u64
    }
}

impl FlagNumConvertible<usize> for bool {
    fn into_bit(self) -> usize {
        to_bit(self)
    }
}

fn to_bit(flag: bool) -> usize {
    if flag {
        1
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use crate::nums::FlagConvertible;

    #[test]
    fn it_convert_to_true() {
        assert!(1u64.is_true());
        assert!(1u32.is_true());
        assert!(1u16.is_true());
        assert!(1u8.is_true());
    }

    #[test]
    fn it_convert_to_false() {
        assert!(0u64.is_false());
        assert!(0u32.is_false());
        assert!(0u16.is_false());
        assert!(0u8.is_false());
    }
}
