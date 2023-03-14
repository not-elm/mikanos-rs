#[repr(transparent)]
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct HeaderType(u8);

impl HeaderType {
    pub fn new(header_type: u8) -> Self {
        Self(header_type)
    }
    pub fn is_multiple_function(&self) -> bool {
        last_bit(self.0) == 1
    }
}

fn last_bit(header_type: u8) -> u8 {
    header_type >> 7
}

#[cfg(test)]
mod tests {
    use crate::pci::configuration_space::common_header::header_type::last_bit;

    #[test]
    fn it_convert_to_header_type() {
        assert_eq!(last_bit(0b1111_0000), 1);
    }
}
