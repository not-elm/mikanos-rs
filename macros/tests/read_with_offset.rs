#![feature(strict_provenance)]

#[cfg(test)]
mod tests {
    use macros::VolatileBits;

    #[test]
    pub fn it_read_with_3bits() {
        #[derive(VolatileBits)]
        #[offset(2)]
        #[volatile_type(u8)]
        struct VolatileStruct(usize);

        let buff = [0b0000_0100u8; 1];
        let addr = buff.as_ptr().addr();
        let v = VolatileStruct::new_uncheck(addr);
        assert_eq!(v.read_volatile(), 0b1);
    }
}
