use proc_macro::TokenStream;

use proc_macro2::{Ident, Span};
use syn::{parse_macro_input, GenericArgument, ItemStruct, PathArguments, Type};

use crate::volatile::attribute::{parse_inner_type, parse_volatile_bits_attributes};
use crate::volatile::impls::{ast_impl_clone, ast_impl_debug};
use crate::volatile::read::{read_flag_volatile, read_volatile};
use crate::volatile::write::write_volatile;

pub mod attribute;
pub mod impls;
pub mod read;
pub mod write;

#[allow(dead_code)]
pub(crate) fn ast_declaration_volatile_accessible(_input: TokenStream) -> TokenStream {
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

pub(crate) fn ast_volatile_bits(input: TokenStream) -> TokenStream {
    let ast_struct = parse_macro_input!(input as ItemStruct);
    let struct_name = ast_struct.clone().ident;
    let (addr_type, phantom_type) = parse_inner_type(ast_struct.clone());

    let (volatile_type, bits, offset, add_addr_bytes) =
        parse_volatile_bits_attributes(ast_struct.clone());
    let volatile_type = volatile_type.unwrap_or(Ident::new("u32", Span::call_site()));

    let read_volatile = read_volatile(volatile_type.clone(), bits.clone(), offset.clone());
    let read_flag_volatile = read_flag_volatile();
    let write_volatile = write_volatile(
        volatile_type.clone(),
        bits.unwrap_or(proc_macro2::Literal::usize_unsuffixed(1)),
        offset.clone(),
    );

    let new_uncheck =
        ast_new_uncheck(addr_type.clone(), add_addr_bytes, phantom_type.clone()).unwrap();

    let impl_debug = ast_impl_debug(struct_name.clone(), volatile_type.clone());
    let impl_clone = ast_impl_clone(struct_name.clone(), phantom_type.clone());

    #[cfg(feature = "extra-traits")]
    let impl_volatile: proc_macro2::TokenStream = ast_impl_volatile_accessible(
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
    let impl_volatile: proc_macro2::TokenStream = ast_impl_volatile_without_trait(
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
fn ast_impl_volatile_accessible(
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
fn ast_impl_volatile_without_trait(
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

fn ast_new_uncheck(
    addr_type: Type,
    add_addr_bytes: proc_macro2::Literal,
    phantom_type: Option<Type>,
) -> Result<proc_macro2::TokenStream, ()> {
    let new = if let Some(_phantom_data) = phantom_type {
        quote::quote! { Self(v + #add_addr_bytes *  core::mem::size_of::<u8>(), PhantomData)}
    } else {
        quote::quote! { Self(v + #add_addr_bytes *  core::mem::size_of::<u8>() )}
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
