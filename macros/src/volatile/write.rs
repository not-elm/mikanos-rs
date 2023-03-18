pub(crate) fn write_volatile(
    volatile_type: proc_macro2::Ident,
    offset: proc_macro2::Literal,
) -> proc_macro2::TokenStream {
    quote::quote! {
        pub fn write_volatile(&self, new_value: #volatile_type) {
            unsafe{core::ptr::write_volatile(self.0 as *mut #volatile_type, (new_value << #offset));}
        }
    }
}
