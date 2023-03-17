use proc_macro2::Ident;

pub(crate) fn impl_hex_debug(struct_name: Ident) -> proc_macro2::TokenStream {
    quote::quote! {
         impl core::fmt::Debug for #struct_name{
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    f.write_fmt(format_args!("{:X}", self.read_volatile()))
                }
         }

        impl core::fmt::UpperHex for #struct_name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                core::fmt::UpperHex::fmt(&self.read_volatile(), f)
            }
        }
    }
}

pub(crate) fn impl_clone(struct_name: Ident) -> proc_macro2::TokenStream {
    quote::quote! {
        impl Clone for #struct_name {
            fn clone(&self) -> Self {
                Self(self.0)
            }
        }
    }
}
