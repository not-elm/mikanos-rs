#![feature(proc_macro_quote)]
#![feature(trace_macros)]
#![no_std]
#[cfg(test)]
extern crate alloc;

use proc_macro::TokenStream;

use proc_macro2::{Ident, Literal, Span};
use syn::{parse_macro_input, GenericArgument, ItemStruct, PathArguments, Type};

use volatile::read::read_volatile;
use volatile::write::write_volatile;

use crate::volatile::attribute::{parse_inner_type, parse_volatile_bits_attributes};
use crate::volatile::impls::{impl_clone, impl_debug};
use crate::volatile::read::read_flag_volatile;

mod generics;
mod volatile;

#[cfg(feature = "extra-traits")]
#[proc_macro]
pub fn declaration_volatile_accessible(_input: TokenStream) -> TokenStream {
    let expand = quote::quote! {
        pub trait VolatileAccessible<ActualValue, Addr, Offset>{
            fn new_uncheck(v: Addr) -> Self;
            fn read_volatile(&self) -> ActualValue;
            fn read_flag_volatile(&self) -> bool;

            fn write_flag_volatile(&self, flag: bool);
            fn write_volatile(&self, value: ActualValue);
        }
    };
    expand.into()
}

#[proc_macro_derive(VolatileBits, attributes(volatile_type, bits, offset_bit))]
pub fn volatile_bits(input: TokenStream) -> TokenStream {
    let struct_ast = parse_macro_input!(input as ItemStruct);
    let struct_name = struct_ast.clone().ident;
    let (addr_type, phantom_type) = parse_inner_type(struct_ast.clone());

    let (volatile_type, bits, offset) = parse_volatile_bits_attributes(struct_ast.clone());
    let volatile_type = volatile_type.unwrap_or(Ident::new("u32", Span::call_site()));

    let read_volatile = read_volatile(volatile_type.clone(), bits.clone(), offset.clone());
    let read_flag_volatile = read_flag_volatile();
    let write_volatile = write_volatile(
        volatile_type.clone(),
        bits.unwrap_or(Literal::usize_unsuffixed(1)),
        offset.clone(),
    );

    let new_uncheck = atr_new_uncheck(addr_type.clone(), phantom_type.clone()).unwrap();

    let impl_debug = impl_debug(struct_name.clone(), volatile_type.clone());
    let impl_clone = impl_clone(struct_name.clone(), phantom_type.clone());

    #[cfg(feature = "extra-traits")]
    let impl_volatile: proc_macro2::TokenStream = impl_volatile_accessible(
        struct_name,
        volatile_type,
        from_phantom(phantom_type),
        addr_type,
        new_uncheck,
        read_volatile,
        read_flag_volatile,
        write_volatile,
    );
    #[cfg(not(feature = "extra-traits"))]
    let impl_volatile: proc_macro2::TokenStream = impl_volatile_without_trait(
        struct_name,
        new_uncheck,
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
    type_param: Ident,
    addr_type: Type,
    new_uncheck: proc_macro2::TokenStream,
    read_volatile: proc_macro2::TokenStream,
    read_flag_volatile: proc_macro2::TokenStream,
    write_volatile: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    quote::quote! {
        use crate::VolatileAccessible;
        impl crate::VolatileAccessible<#volatile_type, #addr_type, #type_param> for #struct_name{
            #new_uncheck
            #read_volatile
            #write_volatile
            #read_flag_volatile
        }
    }
}

#[allow(dead_code)]
fn impl_volatile_without_trait(
    struct_name: Ident,
    new_uncheck: proc_macro2::TokenStream,
    read_volatile: proc_macro2::TokenStream,
    read_flag_volatile: proc_macro2::TokenStream,
    write_volatile: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    quote::quote! {
        impl #struct_name{
            #new_uncheck
            #read_volatile
            #write_volatile
            #read_flag_volatile
        }
    }
}

fn atr_new_uncheck(
    addr_type: Type,
    phantom_type: Option<Type>,
) -> Result<proc_macro2::TokenStream, ()> {
    let new = if let Some(_phantom_data) = phantom_type {
        quote::quote! { Self(v, PhantomData)}
    } else {
        quote::quote! { Self(v)}
    };
    Ok(quote::quote! {
         fn new_uncheck(v: #addr_type) -> Self{
            #new
         }
    })
}

#[allow(dead_code)]
fn from_phantom(phantom_data: Option<Type>) -> Ident {
    if let Some(phantom_data) = phantom_data {
        extract_phantom_type(phantom_data).unwrap()
    } else {
        Ident::new("u32", Span::call_site())
    }
}

#[allow(dead_code)]
fn extract_phantom_type(phantom_data: Type) -> Result<Ident, ()> {
    if let Type::Path(path) = phantom_data {
        let path_segment = path.path.segments.first().unwrap();
        if let PathArguments::AngleBracketed(angle) = path_segment.clone().arguments {
            let generic_argument = angle.args.first().unwrap();
            if let GenericArgument::Type(ty) = generic_argument {
                if let Type::Path(type_path) = ty {
                    return Ok(type_path.path.segments.first().unwrap().ident.clone());
                }
            }
        }
    }

    Err(())
}
