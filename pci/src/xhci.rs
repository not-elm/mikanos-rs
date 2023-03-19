pub mod allocator;
pub mod registers;

pub(crate) fn bit_zero_mask_lower_for(bits: u32, target_value: usize) -> usize {
    let mask = !0 >> (usize::BITS - bits);
    // 下位5Bitsは予約領域
    target_value & !mask
}

#[cfg(test)]
mod tests {
    use crate::xhci::bit_zero_mask_lower_for;

    #[test]
    fn it_mask_lower_3_bits() {
        assert_eq!(bit_zero_mask_lower_for(3, 0b1000_0111), 0b1000_0000);
    }

    #[test]
    fn it_mask_lower_5_bits() {
        let addr = 0b1000_0000_0001_1111;
        assert_eq!(bit_zero_mask_lower_for(5, addr), 0b1000_0000_0000_0000);
    }
}
