//! When deriving `ConstInit` a struct will have a `const_init()` function which
//! allows an initialization at build time.
//!
//! The fields are initialized with a global variable matching the field name in uppercase
//! by default.
//! You can also specify which constant value you want to initialize your field with,
//! using the attribute `#[const_init(value = {constant value/variable})]`.
//!
//! This is basically the `Default` derive macros but with constant values.
//! The goal is to initialize the struct with values that are known at compile time.
//!
//! It is meant to be used with the crate
//! [`const_init_build`](https://docs.rs/const_init_build/latest/const_init_build/index.html)
//!  which helps you create constant variables at build time from a configuration file.
//!
//! # Examples
//!
//! ```rust,ignore
//! use const_init_macros::ConstInit;
//!
//! #[derive(ConstInit)]
//! // This attribute is used to import constant variables from another module.
//! // Here it would import the variables
//! #[const_init(import = generated::settings)]
//! struct FooBar {
//!     // With attribute to specify a constant variable
//!     #[const_init(value = FOO)]
//!     foo: bool,
//!     // Without attribute, looking for matching uppercase field name "BAR"
//!     bar: usize,
//!     #[const_init(value = 3.14)]
//!     toto: f64,
//! }
//! ```
//!
//! # Derived code
//!
//! ```rust,ignore
//! impl FooBar {
//!     use generated::settings::*;
//!     pub const CONST_INIT_VAR: Self = FooBar::const_init();
//!
//!     pub const fn const_init() -> Self {
//!         FooBar {
//!             foo: FOO,
//!             bar: BAR,
//!             toto: 3.14,
//!         }
//!     }
//! }
//! ```
//! Notes: we don't implement a trait `ConstInit` because
//! `const` function in traits are not allowed as of now
//!
//! # Use case
//!
//! ```rust,ignore
//! fn main() {
//!     // Get constant variable for better compiler optimizations
//!     const FOO_BAR: FooBar = FooBar::const_init()
//!     ...
//! }
//! ```
#![allow(dead_code)]
use proc_macro::TokenStream;
use proc_macro2::Span;

mod code_modif;
mod derive_macro;
mod utils;

#[proc_macro_derive(ConstInit, attributes(const_init))]
pub fn derive_const_init(item: TokenStream) -> TokenStream {
    derive_macro::derive_const_init_impl(item)
}

#[proc_macro_attribute]
pub fn const_init_code_modif(attr: TokenStream, item: TokenStream) -> TokenStream {
    match code_modif::const_init_code_modif_impl(attr, item) {
        Ok(ts) => ts,
        Err(e) => syn::Error::new(Span::call_site(), format!("{}", e))
            .into_compile_error()
            .into(),
    }
}
