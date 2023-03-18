#![feature(strict_provenance)]

#[cfg(test)]
mod tests {
    use macros::VolatileBits;

    #[test]
    pub fn it_read_with_3bits() {
        #[derive(VolatileBits)]
        #[bits(3)]
        struct VolatileStruct(usize);

        let buff = [0b100100; 1];
        let addr = buff.as_ptr().addr();
        let v = VolatileStruct::new_uncheck(addr);
        assert_eq!(v.read_volatile(), 0b100);
    }

    #[test]
    pub fn it_read_volatile_u8_with_mask() {
        #[derive(VolatileBits)]
        #[volatile_type(u64)]
        #[bits(3)]
        struct VolatileStruct(usize);

        let buff = [0b100111u64; 1];
        let addr = buff.as_ptr().addr();
        let v = VolatileStruct::new_uncheck(addr);
        assert_eq!(v.read_volatile(), 0b111u64);
    }
}
