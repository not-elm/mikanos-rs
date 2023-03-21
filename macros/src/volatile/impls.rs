use proc_macro2::Ident;
use syn::Type;

pub(crate) fn ast_impl_debug(
    struct_name: Ident,
    volatile_type: proc_macro2::Ident,
) -> proc_macro2::TokenStream {
    if volatile_type == "bool" {
        impl_debug_within_bool(struct_name)
    } else {
        impl_debug_within_upper_hex(struct_name)
    }
}

fn impl_debug_within_upper_hex(struct_name: Ident) -> proc_macro2::TokenStream {
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

fn impl_debug_within_bool(struct_name: Ident) -> proc_macro2::TokenStream {
    quote::quote! {
         impl core::fmt::Debug for #struct_name{
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    f.write_fmt(format_args!("{:?}", self.read_volatile()))
                }
         }
    }
}

pub(crate) fn ast_impl_clone(
    struct_name: Ident,
    phantom_data: Option<Type>,
) -> proc_macro2::TokenStream {
    let new = if let Some(_phantom_data) = phantom_data {
        quote::quote! {  Self(self.0, PhantomData)}
    } else {
        quote::quote! {  Self(self.0)}
    };
    quote::quote! {
        impl Clone for #struct_name {
            fn clone(&self) -> Self {
                #new
            }
        }
    }
}
