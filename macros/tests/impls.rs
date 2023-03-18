#![feature(strict_provenance)]

#[cfg(test)]
mod tests {
    use macros::VolatileBits;

    #[test]
    pub fn it_hex_debug() {
        #[derive(VolatileBits)]
        #[volatile_type(u64)]
        struct VolatileStruct(usize);

        let buff = [0xFFu64; 1];
        let addr = buff.as_ptr().addr();
        let v = VolatileStruct::new_uncheck(addr);
        assert_eq!(format!("{:?}", v), "FF");
    }

    #[test]
    pub fn it_impl_upper_hex() {
        #[derive(VolatileBits)]
        #[volatile_type(u64)]
        struct VolatileStruct(usize);

        let buff = [0xFFu64; 1];
        let addr = buff.as_ptr().addr();
        let v = VolatileStruct::new_uncheck(addr);
        assert_eq!(format!("{:X}", v), "FF");
    }

    #[test]
    pub fn it_impl_clone() {
        #[derive(VolatileBits)]
        #[volatile_type(u64)]
        struct VolatileStruct(usize);

        let buff = [0xFFu64; 1];
        let addr = buff.as_ptr().addr();
        let v = VolatileStruct::new_uncheck(addr);
        assert_eq!(v.clone().read_volatile(), 0xFF);
    }
}
