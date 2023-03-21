#![feature(proc_macro_quote)]
#![feature(trace_macros)]
#![no_std]
#[cfg(test)]
extern crate alloc;

use proc_macro::TokenStream;

mod address;
mod generics;
mod volatile;

#[cfg(feature = "extra-traits")]
#[proc_macro]
pub fn declaration_volatile_accessible(_input: TokenStream) -> TokenStream {
    volatile::ast_declaration_volatile_accessible(_input)
}

#[proc_macro_derive(
    VolatileBits,
    attributes(volatile_type, bits, offset_bit, add_addr_bytes)
)]
pub fn volatile_bits(input: TokenStream) -> TokenStream {
    volatile::ast_volatile_bits(input)
}

#[proc_macro_derive(Address)]
pub fn address(input: TokenStream) -> TokenStream {
    address::ast_impl_new_address(input)
}
