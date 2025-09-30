//! When deriving `ConstInit` a struct will have an init function which fields
//! are initialized with the constant global variable specified by the
//! attributes `#[const_init(value = {expr})].
//! If field attributes
//!
//! This is basically the `Default` derive macros but with constant values.
//! The goal is to initialize the struct with values that are known at compile time.
//!
//! # Examples
//!
//! ```ignore
//! #[derive(ConstInit)]
//! #[const_init(import = "generated::settings::*")]
//! struct FooBar {
//!     // With attribute to specify a constant variable
//!     #[const_init(value = FOO)]
//!     foo: bool,
//!     // Without attribute, looking for matching uppercase field name "BAR"
//!     bar: usize,
//! }
//! ```
//!
//! # Derived code
//!
//! ```ignore
//! impl FooBar {
//!     use generated::settings::*;
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
use syn::{DeriveInput, Expr, Ident};

pub(crate) fn derive_const_init_impl(item: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(item as syn::DeriveInput);
    expand_const_init(ast).into()
}

fn expand_const_init(ast: DeriveInput) -> proc_macro2::TokenStream {
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
        if let Some(const_value) = field.value.as_ref() {
            // If a "value" attribute is indicated, pick this value
            quote! { #field_id: #const_value, }
        } else {
            // else try with the field name in uppercase
            let field_upper = syn::parse_str::<Expr>(&field_id.to_string().to_uppercase())
                .expect("Failed to parse field name into syn::Expr");
            quote! { #field_id: #field_upper, }
        }
    });

    let import_path = if let Some(path) = opts.import_path {
        quote! { use #path::*; }
    } else {
        quote! {}
    };

    let res = quote! {
            impl #struct_id {
                pub const fn const_init() -> Self {
                    #import_path
                    #struct_id {
                        #(#fields_init)*
                    }
                }
            }
    };
    // println!("{}", res.to_string());
    res
}

use darling::{FromDeriveInput, FromField};

#[derive(FromDeriveInput, Debug)]
#[darling(attributes(const_init), supports(struct_named))]
struct ConstInitOpts {
    import_path: Option<Expr>,
    data: darling::ast::Data<darling::util::Ignored, FieldOpts>,
}

#[derive(FromField, Debug)]
#[darling(attributes(const_init))]
struct FieldOpts {
    ident: Option<Ident>,
    value: Option<Expr>,
}
