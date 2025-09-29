#![allow(dead_code)]
use proc_macro::TokenStream;

mod macros;

#[proc_macro_derive(ConstInit, attributes(const_init))]
pub fn derive_const_init(item: TokenStream) -> TokenStream {
    macros::derive_const_init_impl(item)
}
