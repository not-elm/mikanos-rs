#![feature(strict_provenance)]

#[cfg(test)]
mod tests {
    use macros::VolatileBits;

    #[test]
    pub fn it_read_true() {
        #[derive(VolatileBits)]
        struct VolatileStruct(usize);

        let buff = [0b1u32; 1];
        let addr = buff.as_ptr().addr();
        let v = VolatileStruct::new_uncheck(addr);
        assert!(v.read_flag_volatile());
    }

    #[test]
    pub fn it_read_false() {
        #[derive(VolatileBits)]
        struct VolatileStruct(usize);

        let buff = [0b0u32; 1];
        let addr = buff.as_ptr().addr();
        let v = VolatileStruct::new_uncheck(addr);
        assert!(!v.read_flag_volatile());
    }

    #[test]
    pub fn it_read_true_with_offset0() {
        #[derive(VolatileBits)]
        #[offset_bit(0)]
        struct VolatileStruct(usize);

        let buff = [0b0000_0001u32; 1];
        let addr = buff.as_ptr().addr();
        let v = VolatileStruct::new_uncheck(addr);
        assert!(v.read_flag_volatile());
    }

    #[test]
    pub fn it_read_true_with_offset7() {
        #[derive(VolatileBits)]
        #[offset_bit(7)]
        struct VolatileStruct(usize);

        let buff = [0b1000_0000u32; 1];
        let addr = buff.as_ptr().addr();
        let v = VolatileStruct::new_uncheck(addr);
        assert!(v.read_flag_volatile());
    }

    #[test]
    pub fn it_read_true_with_volatile_type() {
        #[derive(VolatileBits)]
        #[volatile_type(u8)]
        struct VolatileStruct(usize);

        let buff = [0b1u32; 1];
        let addr = buff.as_ptr().addr();
        let v = VolatileStruct::new_uncheck(addr);
        assert!(v.read_flag_volatile());
    }

    #[test]
    pub fn it_read_true_with_volatile_type_and_offset() {
        #[derive(VolatileBits)]
        #[volatile_type(u8)]
        #[offset_bit(7)]
        struct VolatileStruct(usize);

        let buff = [0b1000_0000u32; 1];
        let addr = buff.as_ptr().addr();
        let v = VolatileStruct::new_uncheck(addr);
        assert!(v.read_flag_volatile());
    }
}
