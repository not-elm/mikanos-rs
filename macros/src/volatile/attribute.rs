use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{Attribute, ItemStruct, Lit, Meta, MetaList, NestedMeta, Type};

/// Note: NewTypeパターンの構造体(フィールドが1つの場合)を前提
pub(crate) fn parse_inner_type(struct_item: ItemStruct) -> (Type, Option<Type>) {
    let mut iter = struct_item.fields.iter();
    let first_type = iter.next().expect("should be inner field!").clone().ty;
    let second = iter.next().map(|f| f.ty.clone());
    (first_type, second)
}

pub(crate) fn parse_volatile_bits_attributes(
    item_struct: ItemStruct,
) -> (
    Option<proc_macro2::Ident>,
    Option<proc_macro2::Literal>,
    proc_macro2::Literal,
) {
    let mut volatile_type: Option<proc_macro2::Ident> = None;
    let mut bits: Option<proc_macro2::Literal> = None;
    let mut offset: Option<proc_macro2::Literal> = None;
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
            InputAttribute::Offset(v) => {
                offset = Some(v);
            }
        });

    (
        volatile_type.clone(),
        bits,
        offset_suffixed(volatile_type.clone(), offset),
    )
}

fn offset_suffixed(
    volatile_type: Option<proc_macro2::Ident>,
    offset: Option<proc_macro2::Literal>,
) -> proc_macro2::Literal {
    if let Some(offset) = offset {
        return offset;
    }

    if let Some(volatile_type) = volatile_type {
        return if volatile_type == "u8" {
            proc_macro2::Literal::u8_unsuffixed(0)
        } else if volatile_type == "u16" {
            proc_macro2::Literal::u16_unsuffixed(0)
        } else if volatile_type == "u32" {
            proc_macro2::Literal::u32_unsuffixed(0)
        } else if volatile_type == "u64" {
            proc_macro2::Literal::u64_unsuffixed(0)
        } else if volatile_type == "u128" {
            proc_macro2::Literal::u128_unsuffixed(0)
        } else {
            proc_macro2::Literal::usize_unsuffixed(0)
        };
    }

    proc_macro2::Literal::u32_unsuffixed(0)
}

enum InputAttribute {
    VolatileType(proc_macro2::Ident),
    Bits(proc_macro2::Literal),
    Offset(proc_macro2::Literal),
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
    let path_segment = path.segments.first()?;
    let attr_name = path_segment.ident.clone();

    if attr_name == "bits" {
        if let NestedMeta::Lit(Lit::Int(lit)) = nested.first()? {
            return Some(InputAttribute::Bits(lit.token()));
        }
    } else if attr_name == "volatile_type" {
        if let NestedMeta::Meta(Meta::Path(p)) = nested.first()? {
            return Some(InputAttribute::VolatileType(
                p.segments.first()?.ident.clone(),
            ));
        }
    } else if attr_name == "offset" {
        if let NestedMeta::Lit(Lit::Int(lit)) = nested.first()? {
            return Some(InputAttribute::Offset(lit.token()));
        }
    }

    None
}
