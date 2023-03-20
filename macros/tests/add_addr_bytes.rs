#![feature(strict_provenance)]

#[cfg(test)]
mod tests {
    use macros::VolatileBits;

    #[test]
    pub fn it_add_8_address() {
        #[derive(VolatileBits)]
        #[add_addr_bytes(8)]
        #[volatile_type(u8)]
        struct VolatileStruct(usize);

        let buff = [0b0000_0100u8; 1];
        let addr = buff.as_ptr().addr();
        let v = VolatileStruct::new_uncheck(addr);
        assert_eq!(v.0, addr + 0x08);
    }

    #[test]
    pub fn it_read_from_added_0_address() {
        #[derive(VolatileBits)]
        #[add_addr_bytes(0)]
        #[volatile_type(u8)]
        struct VolatileStruct(usize);

        let buff = [0u8, 13u8];
        let addr = buff.as_ptr().addr();
        let v = VolatileStruct::new_uncheck(addr);
        assert_eq!(v.read_volatile(), 0u8);
    }

    #[test]
    pub fn it_read_from_added_8_address() {
        #[derive(VolatileBits)]
        #[add_addr_bytes(1)]
        #[volatile_type(u8)]
        struct VolatileStruct(usize);

        let buff = [0u8, 13u8];
        let addr = buff.as_ptr().addr();
        let v = VolatileStruct::new_uncheck(addr);
        assert_eq!(v.read_volatile(), 13u8);
    }

    #[test]
    pub fn it_when_volatile_32bits_read_from_added_2_address() {
        #[derive(VolatileBits)]
        #[add_addr_bytes(2)]
        #[volatile_type(u32)]
        struct VolatileStruct(usize);

        let buff = [0xF3_FF_00_00u32, 0x00_00_F7_F5u32];
        let addr = buff.as_ptr().addr();
        let v = VolatileStruct::new_uncheck(addr);
        assert_eq!(v.read_volatile(), 0xF7_F5_F3_FF);
    }
}
