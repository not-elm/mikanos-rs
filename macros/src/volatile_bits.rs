use quote::ToTokens;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{Attribute, ItemStruct, Lit, Meta, MetaList, NestedMeta, Type};

pub(crate) fn read_volatile(
    volatile_type: proc_macro2::TokenStream,
    bits: Option<proc_macro2::Literal>,
) -> proc_macro2::TokenStream {
    let r = if let Some(bits) = bits {
        read_volatile_with_mask(volatile_type.clone(), bits)
    } else {
        read_volatile_no_mask(volatile_type.clone())
    };

    quote::quote! {
        pub fn read_volatile(&self) -> #volatile_type{
            #r
        }
    }
}

pub(crate) fn read_volatile_with_mask(
    ty: proc_macro2::TokenStream,
    bits: proc_macro2::Literal,
) -> proc_macro2::TokenStream {
    quote::quote! {
         let mask = !0 >> (#ty::BITS as usize - #bits) ;

         unsafe{core::ptr::read_volatile(self.0 as *const #ty) & mask }
    }
}

pub(crate) fn read_volatile_no_mask(ty: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    quote::quote! {
        unsafe{core::ptr::read_volatile(self.0 as *const #ty)}
    }
}

/// Note: NewTypeパターンの構造体(フィールドが1つの場合)を前提
pub(crate) fn parse_inner_type(struct_item: ItemStruct) -> Type {
    struct_item
        .fields
        .iter()
        .next()
        .expect("should be inner field!")
        .ty
        .clone()
}

pub(crate) fn parse_volatile_bits_attributes(
    item_struct: ItemStruct,
) -> (
    Option<proc_macro2::TokenStream>,
    Option<proc_macro2::Literal>,
) {
    let mut volatile_type: Option<proc_macro2::TokenStream> = None;
    let mut bits: Option<proc_macro2::Literal> = None;
    item_struct
        .attrs
        .iter()
        .map(|attr| parse_attribute(attr.clone()))
        .filter_map(|v| v)
        .for_each(|input_attribute| match input_attribute {
            InputAttribute::Bits(input_bits) => {
                bits = Some(input_bits);
            }
            InputAttribute::VolatileType(v) => {
                volatile_type = Some(v);
            }
        });

    (volatile_type, bits)
}

enum InputAttribute {
    VolatileType(proc_macro2::TokenStream),
    Bits(proc_macro2::Literal),
}

fn parse_attribute(attr: Attribute) -> Option<InputAttribute> {
    if let Ok(meta) = attr.parse_meta() {
        if let Meta::List(MetaList {
            ref path,
            paren_token: _,
            ref nested,
        }) = meta
        {
            return parse_meta_name_value(path, nested);
        }

        return None;
    }

    None
}

fn parse_meta_name_value(
    path: &syn::Path,
    nested: &Punctuated<NestedMeta, Comma>,
) -> Option<InputAttribute> {
    let p = path.segments.first()?;
    let attr_name = p.ident.clone();

    if attr_name == "bits" {
        if let NestedMeta::Lit(Lit::Int(lit)) = nested.first()? {
            return Some(InputAttribute::Bits(lit.token()));
        }
    } else if attr_name == "volatile_type" {
        if let NestedMeta::Meta(Meta::Path(p)) = nested.first()? {
            return Some(InputAttribute::VolatileType(
                p.segments.first()?.ident.clone().into_token_stream(),
            ));
        }
        return None;
    }

    None
}