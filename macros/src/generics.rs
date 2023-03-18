use syn::{GenericParam, Generics, TypeParam};

#[allow(dead_code)]
pub fn find_type_param(generics: Generics) -> Option<TypeParam> {
    generics.params.iter().find_map(|param| {
        if let GenericParam::Type(type_param) = param.clone() {
            Some(type_param)
        } else {
            None
        }
    })
}
