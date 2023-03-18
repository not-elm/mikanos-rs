use macros::VolatileBits;

#[test_case]
fn it_write() {
    #[derive(VolatileBits)]
    #[volatile_type(u64)]
    struct VolatileStruct(usize);

    let addr = [0x00u64; 3].as_ptr().addr();
    let v = VolatileStruct(addr);
    assert_eq!(v.read_volatile(), 0x00);
    v.write_volatile(0xFF);

    assert_eq!(v.read_volatile(), 0xFF);
}

#[test_case]
fn it_write_with_bits() {
    #[derive(VolatileBits)]
    #[bits(2)]
    #[volatile_type(u64)]
    struct VolatileStruct(usize);

    let addr = [0b1000u64; 1].as_ptr().addr();
    let v = VolatileStruct::new_uncheck(addr);
    v.write_volatile(0b11);

    assert_eq!(v.read_volatile(), 0b11);
}

#[test_case]
fn it_write_with_offset() {
    #[derive(VolatileBits)]
    #[offset(1)]
    #[volatile_type(u64)]
    struct VolatileStruct(usize);

    let addr = [0b11u64; 1].as_ptr().addr();
    let v = VolatileStruct::new_uncheck(addr);
    v.write_volatile(0b0);

    assert_eq!(v.read_volatile(), 0b0);
}

#[test_case]
fn it_write_with_bits_and_offset() {
    #[derive(VolatileBits)]
    #[bits(2)]
    #[offset(1)]
    #[volatile_type(u8)]
    struct VolatileStruct(usize);
    let buff = [0b1000u8; 1];
    let addr = buff.as_ptr().addr();
    let v = VolatileStruct::new_uncheck(addr);
    v.write_volatile(0b11);

    assert_eq!(v.read_volatile(), 0b11);
    assert_eq!(buff[0], 0b1110)
}

#[test_case]
fn it_write_with_bits_and_offset16() {
    #[derive(VolatileBits)]
    #[bits(4)]
    #[offset(8)]
    #[volatile_type(u16)]
    struct VolatileStruct(usize);
    let buff = [0b1001_0101_1100_0000u16; 1];
    let addr = buff.as_ptr().addr();
    let v = VolatileStruct::new_uncheck(addr);
    v.write_volatile(0b00);

    assert_eq!(v.read_volatile(), 0b00);
    assert_eq!(buff[0], 0b1001_0000_1100_0000u16)
}

#[test_case]
fn it_write_flag() {
    #[derive(VolatileBits)]
    #[bits(3)]
    #[offset(8)]
    #[volatile_type(u16)]
    struct VolatileStruct(usize);
    let buff = [0b1000_0001_0000_0000u16; 1];
    let addr = buff.as_ptr().addr();
    let v = VolatileStruct::new_uncheck(addr);

    assert!(v.read_flag_volatile());
    v.write_flag_volatile(false);

    assert!(!v.read_flag_volatile());
    assert_eq!(buff[0], 0b1000_0000_0000_0000u16)
}
