pub(crate) fn write_volatile(
    volatile_type: proc_macro2::Ident,
    bits: proc_macro2::Literal,
    offset: proc_macro2::Literal,
) -> proc_macro2::TokenStream {
    quote::quote! {

        fn write_flag_volatile(&self, flag: bool) {
            if flag{
                self.write_volatile(1)
            }else{
                self.write_volatile(0)
            }
        }

        fn write_volatile(&self, new_value: #volatile_type) {
            let shift = (new_value as usize) << #offset;
            let mask = !(0 as usize)  >> (#volatile_type::BITS as usize - #bits);
            let mask = mask << #offset;
            let mask = !(mask);
            let all_bits = unsafe{core::ptr::read_volatile(self.0 as *const usize) & mask };

            unsafe{core::ptr::write_volatile(self.0 as *mut usize, all_bits | shift);}
        }
    }
}
