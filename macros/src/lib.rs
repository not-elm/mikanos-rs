#![feature(proc_macro_quote)]
#![feature(trace_macros)]

#[cfg(test)]
extern crate alloc;

use proc_macro::TokenStream;

use proc_macro2::{Ident, Literal, Span};
use syn::{parse_macro_input, ItemStruct};

use volatile::read::read_volatile;
use volatile::write::write_volatile;

use crate::volatile::attribute::{parse_inner_type, parse_volatile_bits_attributes};
use crate::volatile::impls::{impl_clone, impl_debug};
use crate::volatile::read::read_flag_volatile;

mod volatile;

#[proc_macro_derive(VolatileBits, attributes(volatile_type, bits, offset))]
pub fn volatile_bits(input: TokenStream) -> TokenStream {
    let item_struct = parse_macro_input!(input as ItemStruct);
    let struct_name = item_struct.clone().ident;
    let inner_ty = parse_inner_type(item_struct.clone());

    let (volatile_type, bits, offset) = parse_volatile_bits_attributes(item_struct.clone());
    let volatile_type = volatile_type.unwrap_or(Ident::new("u32", Span::call_site()));
    let offset = offset.unwrap_or(Literal::u32_unsuffixed(0));
    let read_volatile = read_volatile(volatile_type.clone(), bits, offset.clone());
    let read_flag_volatile = read_flag_volatile();
    let write_volatile = write_volatile(volatile_type.clone(), offset.clone());

    let impl_debug = impl_debug(struct_name.clone(), volatile_type.clone());
    let impl_clone = impl_clone(struct_name.clone());

    let expand = quote::quote! {
        impl #struct_name{
            pub fn new_uncheck(v: #inner_ty) -> Self{
                Self(v)
            }


            #read_volatile
            #read_flag_volatile
            #write_volatile
        }

        #impl_debug
        #impl_clone
    };

    expand.into()
}
