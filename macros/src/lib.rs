#![feature(proc_macro_quote)]
#![feature(trace_macros)]

#[cfg_attr(not(test), no_std)]
#[cfg(test)]
extern crate alloc;

use proc_macro::TokenStream;

use proc_macro2::{Ident, Span};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{
    parse_macro_input, Attribute, ItemStruct, Lit, Meta, MetaList, MetaNameValue, NestedMeta,
};

mod flag;

#[proc_macro_derive(VolatileFlag)]
pub fn flag(input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as ItemStruct);
    let struct_name = item.clone().ident;
    let ty = item
        .fields
        .iter()
        .next()
        .expect("フィールドが宣言されていません。");
    let out_ty = proc_macro2::Ident::new("bool", Span::call_site());
    let read_volatile = read_volatile(out_ty.clone(), None);

    let write_volatile = write_volatile(out_ty.clone(), None);
    let gen = quote::quote! {
        impl #struct_name{
            pub fn new(t: #ty) -> Self{
                Self(t)
            }
            pub fn new_expect_to_be(is_true: bool, t: #ty) -> core::option::Option<Self>{
                let me = Self::new(t);
                if is_true == me.read_volatile(){
                    core::option::Option::Some(me)
                }else{
                    core::option::Option::None
                }
            }

            pub fn read_volatile(&self) -> #out_ty{
                #read_volatile
            }

            pub fn write_volatile(&self, value: #out_ty){
                #write_volatile
            }

        }

         impl core::fmt::Debug for #struct_name{
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    f.write_fmt(format_args!("{:?}", self.0))
                }
         }

        impl Clone for #struct_name {
            fn clone(&self) -> Self {
                Self(self.0)
            }
        }
    };

    gen.into()
}

#[proc_macro_derive(Volatile, attributes(volatile_type))]
pub fn impl_volatile(input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as ItemStruct);
    let struct_name = item.clone().ident;
    let ty = item
        .fields
        .iter()
        .next()
        .expect("フィールドが宣言されていません。");
    let attribute = item.attrs[0].clone();

    let (out_ty, right_shift) = parse_attr_each(attribute);

    let read_volatile = read_volatile(out_ty.clone(), right_shift.clone());

    let write_volatile = write_volatile(out_ty.clone(), right_shift.clone());

    let ty = &ty.ty;

    let gen = quote::quote! {
        impl #struct_name{
            pub fn new(t: #ty) -> Self{
                Self(t)
            }
            pub fn new_non_zero(t: #ty) -> core::option::Option<Self>{
                let me = Self::new(t);
                if 0 < me.read_volatile(){
                    core::option::Option::Some(me)
                }else{
                    core::option::Option::None
                }
            }

            pub fn read_volatile(&self) -> #out_ty{
                #read_volatile
            }

            pub fn write_volatile(&self, value: #out_ty){
                #write_volatile
            }

        }

    };

    gen.into()
}

fn read_volatile(
    out_ty: Ident,
    right_shift: Option<proc_macro2::Literal>,
) -> proc_macro2::TokenStream {
    if let Some(right_shift) = right_shift {
        quote::quote! {
            unsafe{core::ptr::read_volatile(self.0 as *const #out_ty) >> #right_shift}
        }
    } else {
        quote::quote! {
            unsafe{core::ptr::read_volatile(self.0 as *const #out_ty)}
        }
    }
}

fn write_volatile(
    out_ty: Ident,
    right_shift: Option<proc_macro2::Literal>,
) -> proc_macro2::TokenStream {
    if let Some(right_shift) = right_shift {
        quote::quote! {
             unsafe{core::ptr::write_volatile(self.0 as *mut #out_ty, value << #right_shift)}
        }
    } else {
        quote::quote! {
            unsafe{core::ptr::write_volatile(self.0 as *mut #out_ty, value)}
        }
    }
}

fn parse_attr_each(attr: Attribute) -> (Ident, Option<proc_macro2::Literal>) {
    match attr.parse_meta() {
        Ok(meta) => match meta {
            Meta::List(MetaList {
                path: _,
                paren_token: _,
                ref nested,
            }) => parse_nested(nested),
            _ => panic!("Should be Attribute"),
        },
        _ => panic!("Should be Attribute"),
    }
}

fn parse_nested(nested: &Punctuated<NestedMeta, Comma>) -> (Ident, Option<proc_macro2::Literal>) {
    let mut ty: Option<Ident> = None;
    let mut right_shift: Option<proc_macro2::Literal> = None;
    for n in nested {
        match n {
            NestedMeta::Meta(Meta::Path(p)) => {
                ty = Some(
                    p.segments
                        .first()
                        .expect("should be volatile_type")
                        .ident
                        .clone(),
                )
            }
            NestedMeta::Meta(Meta::NameValue(MetaNameValue {
                ref path,
                eq_token: _,
                ref lit,
            })) => {
                if path.segments.first().unwrap().ident == "right_shift" {
                    if let Lit::Int(shift) = lit {
                        right_shift = Some(shift.token())
                    }
                }
            }
            _ => {}
        }
    }

    (ty.expect("should be volatile_type"), right_shift)
}
