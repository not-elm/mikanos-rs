pub(crate) fn write_volatile(
    volatile_type: proc_macro2::Ident,
    bits: proc_macro2::Literal,
    offset: proc_macro2::Literal,
) -> proc_macro2::TokenStream {
    quote::quote! {

        pub fn write_flag_volatile(&self, flag: bool) {
            if flag{
                self.write_volatile(1)
            }else{
                self.write_volatile(0)
            }
        }

        pub fn write_volatile(&self, new_value: #volatile_type) {
            let shift = new_value << #offset;
            let mask = (!0 as #volatile_type) >> (#volatile_type::BITS as usize - #bits);
            let mask = mask << #offset;
            let mask = !(mask  as #volatile_type);
            let all_bits = unsafe{core::ptr::read_volatile(self.0 as *const #volatile_type) & mask };

            unsafe{core::ptr::write_volatile(self.0 as *mut #volatile_type, all_bits | shift);}
        }
    }
}
