#![feature(type_name_of_val)]
#![feature(strict_provenance)]
#![feature(is_some_and)]

#[cfg(test)]
mod tests {
    use macros::{Volatile, VolatileFlag};

    #[test]
    fn it_impl_read_volatile() {
        #[derive(Volatile)]
        #[volatile_type(u32)]
        struct VolatileStruct(usize);

        let addr = [0x33u32; 1].as_ptr().addr();
        let v = VolatileStruct(addr);
        assert_eq!(core::any::type_name_of_val(&v.read_volatile()), "u32");
        assert_eq!(v.read_volatile(), 0x33);
    }

    #[test]
    fn it_impl_read_volatile_u32() {
        #[derive(Volatile)]
        #[volatile_type(u64)]
        struct VolatileStruct(usize);

        let addr = [0xFFu64; 1].as_ptr().addr();
        let v = VolatileStruct(addr);
        assert_eq!(core::any::type_name_of_val(&v.read_volatile()), "u64");
        assert_eq!(v.read_volatile(), 0xFF);
    }

    #[test]
    fn it_new_non_zero() {
        #[derive(Volatile)]
        #[volatile_type(u64)]
        struct VolatileStruct(usize);

        let addr = [0xFFu64; 1].as_ptr().addr();
        let v = VolatileStruct::new_non_zero(addr);

        assert!(v.is_some());
    }

    #[test]
    fn it_is_none() {
        #[derive(Volatile)]
        #[volatile_type(u64)]
        struct VolatileStruct(usize);

        let addr = [0u64; 1].as_ptr().addr();
        let v = VolatileStruct::new_non_zero(addr);

        assert!(v.is_none());
    }

    #[test]
    fn it_twice_shift() {
        #[derive(Volatile)]
        #[volatile_type(u64, right_shift = 2)]
        struct VolatileStruct(usize);

        let addr = [0b1100u64; 1].as_ptr().addr();
        let v = VolatileStruct::new(addr);

        assert_eq!(v.read_volatile(), 0b11);
    }

    #[test]
    fn it_is_true() {
        #[derive(VolatileFlag)]
        struct VolatileStruct(usize);

        let addr = [0b1000u64; 1].as_ptr().addr();
        let v = VolatileStruct::new(addr);

        assert!(!v.read_volatile());
    }

    #[test]
    fn it_is_false() {
        #[derive(VolatileFlag)]
        struct VolatileStruct(usize);

        let addr = [0b1001u64; 1].as_ptr().addr();
        let v = VolatileStruct::new(addr);

        assert!(v.read_volatile());
    }

    #[test]
    fn it_is_some() {
        #[derive(VolatileFlag)]
        struct VolatileStruct(usize);

        let addr = [0b1001u64; 1].as_ptr().addr();
        let v = VolatileStruct::new_expect_to_be(true, addr);

        assert!(v.is_some_and(|x| x.read_volatile()));
    }

    #[test]
    fn it_is_none_flag() {
        #[derive(VolatileFlag)]
        struct VolatileStruct(usize);

        let addr = [0b1000u64; 1].as_ptr().addr();
        let v = VolatileStruct::new_expect_to_be(true, addr);

        assert!(v.is_none());
    }

    #[test]
    fn it_clone() {
        #[derive(VolatileFlag)]
        struct VolatileStruct(usize);

        let addr = [0b1000u64; 1].as_ptr().addr();
        let v = VolatileStruct::new(addr).clone();

        assert!(!v.read_volatile());
    }
}
