use macros::VolatileBits;

#[test_case]
fn it_add_1_byte() {
    #[derive(VolatileBits)]
    #[volatile_type(u8)]
    #[add_addr_bytes(1)]
    struct VolatileStruct(usize);
    let buff = [0xFFu8, 0x01u8, 0xFFu8];
    let addr = buff.as_ptr().addr();
    let v = VolatileStruct::new_uncheck(addr);

    v.write_volatile(0x33);
    assert_eq!(v.read_volatile(), 0x33);
    assert_eq!(buff[1], 0x33);
}

#[test_case]
fn it_add_3_bytes_with_offset() {
    #[derive(VolatileBits)]
    #[volatile_type(u32)]
    #[add_addr_bytes(4)]
    #[offset_bit(3)]
    struct VolatileStruct(usize);
    let buff = [
        0b0000_0000_0000_0000__0000_0000_0000_0000u32,
        0b0000_0000_0000_0000__0000_0000_0000_0000u32,
        0b0000_0000_0000_0000__0000_0000_0000_0000u32,
    ];
    let addr = buff.as_ptr().addr();
    let v = VolatileStruct::new_uncheck(addr);

    v.write_volatile(0b1111_1111_1111);
    assert_eq!(v.read_volatile(), 0b1111_1111_1111);
    assert_eq!(buff[0], 0b0000_0000_0000_0000__0000_0000_0000_0000u32);
    assert_eq!(buff[1], 0b0000_0000_0000_0000__0111_1111_1111_1000u32);
}
