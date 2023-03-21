#[cfg(test)]
mod tests {
    use macros::Address;

    #[test]
    pub fn it_impl_new_and_addr() {
        #[derive(Address)]
        struct TestAddress(usize);

        let addr = TestAddress::new(0x03);
        assert_eq!(addr.addr(), 0x03);
    }
}
