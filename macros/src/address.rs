use proc_macro::TokenStream;

use syn::{parse_macro_input, ItemStruct};

use crate::volatile::attribute::parse_inner_type;

pub(crate) fn ast_impl_new_address(input: TokenStream) -> TokenStream {
    let ast_struct = parse_macro_input!(input as ItemStruct);
    let struct_name = ast_struct.clone().ident;
    let (addr_type, _) = parse_inner_type(ast_struct);

    let expand = quote::quote! {
        impl #struct_name {
            pub fn new(addr: #addr_type) -> Self {
                Self(addr)
            }

            pub fn addr(&self) -> #addr_type {
                self.0
            }
        }
    };

    expand.into()
}
