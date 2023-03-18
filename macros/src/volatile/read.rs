pub(crate) fn read_volatile(
    volatile_type: proc_macro2::Ident,
    bits: Option<proc_macro2::Literal>,
    offset: proc_macro2::Literal,
) -> proc_macro2::TokenStream {
    let r = if let Some(bits) = bits {
        read_volatile_with_mask(volatile_type.clone(), bits, offset)
    } else {
        read_volatile_no_mask(volatile_type.clone(), offset)
    };

    quote::quote! {
        pub fn read_volatile(&self) -> #volatile_type{
            #r
        }
    }
}

pub(crate) fn read_flag_volatile() -> proc_macro2::TokenStream {
    quote::quote! {
        pub fn read_flag_volatile(&self) -> bool{
            let lsb  = self.read_volatile() & 0b1;
            return if lsb == 1{
                true
            }else{
                false
            }
        }
    }
}

fn read_volatile_with_mask(
    ty: proc_macro2::Ident,
    bits: proc_macro2::Literal,
    offset: proc_macro2::Literal,
) -> proc_macro2::TokenStream {
    quote::quote! {
         let mask = !0 >> (#ty::BITS as usize - #bits) ;

         unsafe{(core::ptr::read_volatile(self.0 as *const #ty) >> #offset as #ty) & mask }
    }
}

pub(crate) fn read_volatile_no_mask(
    ty: proc_macro2::Ident,
    offset: proc_macro2::Literal,
) -> proc_macro2::TokenStream {
    quote::quote! {
        unsafe{core::ptr::read_volatile(self.0 as *const #ty) >> #offset as #ty}
    }
}
