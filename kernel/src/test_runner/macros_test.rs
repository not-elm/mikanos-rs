use macros::Volatile;

#[test_case]
fn it_impl_write_volatile() {
    #[derive(Volatile)]
    #[volatile_type(u64)]
    struct VolatileStruct(usize);

    let addr = [0x00u64; 3].as_ptr().addr();
    let v = VolatileStruct(addr);
    assert_eq!(v.read_volatile(), 0x00);
    v.write_volatile(0xFF);

    assert_eq!(v.read_volatile(), 0xFF);
}

#[test_case]
fn it_twice_shift_write() {
    #[derive(Volatile)]
    #[volatile_type(u64, right_shift = 3)]
    struct VolatileStruct(usize);

    let addr = [0b00u64; 1].as_ptr().addr();
    let v = VolatileStruct::new(addr);
    v.write_volatile(0b11);

    assert_eq!(v.read_volatile(), 0b11);
}
