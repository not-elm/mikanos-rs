#![feature(proc_macro_quote)]
#![feature(trace_macros)]

#[cfg(test)]
extern crate alloc;

use proc_macro::TokenStream;

use proc_macro2::{Ident, Literal, Span};
use syn::{parse_macro_input, ItemStruct, Type};

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
    let addr_type = parse_inner_type(item_struct.clone());

    let (volatile_type, bits, offset) = parse_volatile_bits_attributes(item_struct.clone());
    let volatile_type = volatile_type.unwrap_or(Ident::new("u32", Span::call_site()));

    let read_volatile = read_volatile(volatile_type.clone(), bits.clone(), offset.clone());
    let read_flag_volatile = read_flag_volatile();
    let write_volatile = write_volatile(
        volatile_type.clone(),
        bits.unwrap_or(Literal::usize_unsuffixed(1)),
        offset.clone(),
    );

    let impl_debug = impl_debug(struct_name.clone(), volatile_type.clone());
    let impl_clone = impl_clone(struct_name.clone());
    #[cfg(feature = "extra-traits")]
    let impl_volatile: proc_macro2::TokenStream = impl_volatile_accessible(
        struct_name,
        volatile_type,
        addr_type,
        read_volatile,
        read_flag_volatile,
        write_volatile,
    );
    #[cfg(not(feature = "extra-traits"))]
    let impl_volatile: proc_macro2::TokenStream = impl_volatile_without_trait(
        struct_name,
        addr_type,
        read_volatile,
        read_flag_volatile,
        write_volatile,
    );

    let expand = quote::quote! {
        #impl_volatile

        #impl_debug
        #impl_clone
    };

    expand.into()
}

#[allow(dead_code)]
fn impl_volatile_accessible(
    struct_name: Ident,
    volatile_type: Ident,
    addr_type: Type,
    read_volatile: proc_macro2::TokenStream,
    read_flag_volatile: proc_macro2::TokenStream,
    write_volatile: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    quote::quote! {
        use common_lib::volatile_accessible::VolatileAccessible;

        impl common_lib::volatile_accessible::VolatileAccessible<#volatile_type, #addr_type> for #struct_name{
            fn new_uncheck(v: #addr_type) -> Self{
                Self(v)
            }
            #read_volatile
            #write_volatile
            #read_flag_volatile
        }
    }
}

#[allow(dead_code)]
fn impl_volatile_without_trait(
    struct_name: Ident,
    addr_type: Type,
    read_volatile: proc_macro2::TokenStream,
    read_flag_volatile: proc_macro2::TokenStream,
    write_volatile: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    quote::quote! {
        impl #struct_name{
            fn new_uncheck(v: #addr_type) -> Self{
                Self(v)
            }
            #read_volatile
            #write_volatile
            #read_flag_volatile
        }
    }
}
