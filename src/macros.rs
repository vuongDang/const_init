//! When deriving `ConstInit` a struct will have an init function which fields
//! are initialized with the constant global variable specified by the
//! attributes `#[const_init(value = {expr})]
//!
//! This is basically the `Default` derive macros but the goal
//! is to initialize the struct with values that are only known at compile time.
//!
//! # Examples
//!
//! ```ignore
//! #[derive(ConstInit)]
//! struct FooBar {
//!     #[const_init(value = FOO)]
//!     foo: bool,
//!     #[const_init(value = BAR)]
//!     bar: usize,
//!     other: usize,
//! }
//! ```
//!
//! # Derived code
//!
//! ```ignore
//! impl FooBar {
//!     pub const fn const_init() -> Self {
//!         FooBar {
//!             foo: FOO,
//!             bar: BAR,
//!         }
//!     }
//! }
//! ```
//! Notes: we don't implement a trait `ConstInit` because
//! const function in traits are not allowed as of now

use proc_macro::TokenStream;
use quote::quote;
use syn::{Expr, Ident};

pub(crate) fn derive_const_init_impl(item: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(item as syn::DeriveInput);
    let struct_id = &ast.ident;

    let opts = match ConstInitOpts::from_derive_input(&ast) {
        Ok(val) => val,
        Err(e) => return e.write_errors().into(),
    };

    let fields = opts
        .data
        .as_ref()
        .take_struct()
        .expect("ConstInit can only be derived for structs")
        .fields;

    let fields_init = fields.iter().map(|field| {
        let field_id = field.ident.as_ref().unwrap();
        let const_value = field.value.as_ref().unwrap();
        quote! { #field_id: #const_value, }
    });

    let res = quote! {
            impl #struct_id {
                pub const fn const_init() -> Self {
                    #struct_id {
                        #(#fields_init)*
                    }
                }
            }
    };
    res.into()
}

use darling::{FromDeriveInput, FromField};

#[derive(FromDeriveInput, Debug)]
#[darling(attributes(const_init), supports(struct_named))]
struct ConstInitOpts {
    data: darling::ast::Data<darling::util::Ignored, FieldOpts>,
}

#[derive(FromField, Debug)]
#[darling(attributes(const_init))]
struct FieldOpts {
    ident: Option<Ident>,
    value: Option<Expr>,
}
