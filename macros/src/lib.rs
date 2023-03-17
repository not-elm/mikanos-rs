#![feature(proc_macro_quote)]
#![feature(trace_macros)]
#![no_std]
#[cfg(test)]
extern crate alloc;

use proc_macro::TokenStream;

use proc_macro2::Ident;
use syn::{parse_macro_input, Attribute, ItemStruct, Meta, MetaList, NestedMeta};

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

    let out_ty = parse_attr_each(attribute).expect("should be volatile_type");

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
                unsafe{core::ptr::read_volatile(self.0 as *const #out_ty)}
            }

            pub fn write_volatile(&self, value: #out_ty){
                unsafe{core::ptr::write_volatile(self.0 as *mut #out_ty, value)}
            }

        }

    };

    gen.into()
}

fn parse_attr_each(attr: Attribute) -> core::option::Option<Ident> {
    match attr.parse_meta() {
        Ok(meta) => match meta {
            Meta::List(MetaList {
                path: _,
                paren_token: _,
                ref nested,
            }) => {
                if let NestedMeta::Meta(Meta::Path(p)) = nested.first()? {
                    core::option::Option::Some(
                        p.segments
                            .first()
                            .expect("should be volatile_type")
                            .ident
                            .clone(),
                    )
                } else {
                    core::option::Option::None
                }
            }
            _ => core::option::Option::None,
        },
        _ => core::option::Option::None,
    }
}
