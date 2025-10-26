#![allow(dead_code)]
#![doc = include_str!("../../../docs/const_init_macros/overview.md")]
//! # Full example
//! ```rust,ignore
#![doc = include_str!("../../../examples/full_example.rs")]
//! ```
use proc_macro::TokenStream;
use proc_macro2::Span;

mod code_modif;
mod derive_macro;
mod utils;

#[doc = include_str!("../../../docs/const_init_macros/ConstInit_derive_macro.md")]
#[proc_macro_derive(ConstInit, attributes(const_init))]
pub fn derive_const_init(item: TokenStream) -> TokenStream {
    derive_macro::derive_const_init_impl(item)
}

#[doc = include_str!("../../../docs/const_init_macros/code_modif.md")]
#[proc_macro_attribute]
pub fn const_init_code_modif(attr: TokenStream, item: TokenStream) -> TokenStream {
    match code_modif::const_init_code_modif_impl(attr, item) {
        Ok(ts) => ts,
        Err(e) => syn::Error::new(Span::call_site(), format!("{}", e))
            .into_compile_error()
            .into(),
    }
}
