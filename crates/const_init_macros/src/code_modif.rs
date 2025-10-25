use anyhow::Context;
use darling::{Error, ast::NestedMeta};
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{ToTokens, quote};
use syn::{Block, FnArg, ItemFn, Signature, Stmt, Token, Type, spanned::Spanned};

pub(crate) fn const_init_code_modif_impl(
    attr: TokenStream,
    item: TokenStream,
) -> anyhow::Result<TokenStream> {
    // Gather arguments from attributes
    let attr_args: Vec<NestedMeta> = NestedMeta::parse_meta_list(attr.into())?;
    let attr_args = ConstInitArgs::from_list(&attr_args)
        .map_err(|e| anyhow::anyhow!("Attribute parsing error: {e}"))
        .context("while parsing #[const_init(...)] arguments")?;

    // Gather info from the attributes
    let _is_return_const = attr_args.const_return;
    let idents_of_const_params = attr_args.target_params.0;
    let fn_noop = attr_args.noop;
    let fn_replace = attr_args.replace_with_const_init;

    // Gather info from the function ast
    let item_fn: ItemFn = syn::parse(item)?;

    let expanded = if fn_noop {
        replace_whole_fn_with_noop(&item_fn)?
    } else if fn_replace {
        replace_whole_fn_with_const_init(&item_fn)?
    } else {
        init_target_params_with_const_init(&item_fn, &idents_of_const_params)?
    };

    Ok(TokenStream::from(expanded))
}

// Replace the whole function block with a noop
// This is used to replace function taking a mutable reference that
// we won't mutate in `const` mode
// Only works if the return type is unit type
// NOTE: if your function has side-effects this will remove all of them
fn replace_whole_fn_with_noop(item_fn: &ItemFn) -> syn::Result<proc_macro2::TokenStream> {
    let vis = &item_fn.vis;
    let sig = &item_fn.sig;
    let new_sig = transform_to_const_fn(&sig);
    match &sig.output {
        syn::ReturnType::Default => Ok(quote! {
            #vis #new_sig {
            }
        }),
        syn::ReturnType::Type(_, return_typ) => Err(syn::Error::new(
            Span::call_site(),
            format!("This return type is not handled {:?}", *return_typ),
        )
        .into()),
    }
}

// Replace the whole function block w
fn replace_whole_fn_with_const_init(item_fn: &ItemFn) -> syn::Result<proc_macro2::TokenStream> {
    let vis = &item_fn.vis;
    let sig = &item_fn.sig;
    let return_typ = get_info_of_const_return(&sig)?;
    let new_sig = transform_to_const_fn(&sig);
    match return_typ {
        Some(ReturnInfo {
            param_kind: ParamKind::Owned,
            param_type,
        }) => {
            let param_type: Type = syn::parse_str(&param_type)?;
            Ok(quote! {
                #vis #new_sig {
                    #param_type::const_init()
                }
            })
        }
        _ => Err(syn::Error::new(
            Span::call_site(),
            format!("This return type is not handled {:?}", return_typ),
        )
        .into()),
    }
}

fn init_target_params_with_const_init(
    item_fn: &ItemFn,
    idents_of_const_params: &Vec<String>,
) -> syn::Result<proc_macro2::TokenStream> {
    let block = &item_fn.block;
    let vis = &item_fn.vis;
    let sig = &item_fn.sig;
    // Gather info of the const params
    let const_params_info = get_info_of_const_params(idents_of_const_params, &sig)?;
    // let return_info: Option<ReturnInfo> = if is_return_const {
    //     get_info_of_const_return(&sig)?
    // } else {
    //     None
    // };

    // Change mutability of the const parameters
    let new_sig = set_const_init_params_to_mutable_in_sig(sig, &const_params_info)?;
    let new_block = generate_code_block(&const_params_info, block)?;

    Ok(quote! {
        #vis #new_sig
            #new_block

    })
}

// Transform function to `const fn` if not already constant
fn transform_to_const_fn(sig: &Signature) -> Signature {
    let mut new_sig = sig.clone();
    if new_sig.constness.is_none() {
        new_sig.constness = Some(Token![const](Span::call_site()));
    }
    new_sig
}

fn generate_code_block(const_params_info: &[ParamInfo], block: &Box<Block>) -> syn::Result<Block> {
    let mut new_block = *block.clone();
    for const_param in const_params_info {
        let param_ident = &const_param.ident;
        let param_type = &const_param.param_type;
        let code = match const_param.param_kind {
            ParamKind::Owned => {
                syn::parse_str::<Stmt>(&format!("{param_ident} = {param_type}::CONST_INIT_VAR;"))?
            }
            ParamKind::SharedRef => {
                syn::parse_str::<Stmt>(&format!("{param_ident} = &{param_type}::CONST_INIT_VAR;"))?
            }
            ParamKind::MutRef => {
                syn::parse_str::<Stmt>(&format!("*{param_ident} = {param_type}::CONST_INIT_VAR;"))?
            }
        };
        new_block.stmts.insert(0, code);
    }

    Ok(new_block)
}

fn get_info_of_const_params(
    idents_of_const_params: &Vec<String>,
    sig: &Signature,
) -> syn::Result<Vec<ParamInfo>> {
    // Get information on all the parameters of the signature
    let params_info = sig
        .inputs
        .iter()
        .map(|fn_arg| ParamInfo::from_fn_arg(fn_arg))
        .collect::<anyhow::Result<Vec<ParamInfo>, _>>()?;

    // Pick only the params info of const parameters.
    // Create an error if a const parameter is not found
    idents_of_const_params
        .iter()
        .map(|ident| {
            params_info
                .iter()
                .find(|param_info| param_info.ident == ident.to_string())
                .ok_or(syn::Error::new(
                    ident.span(),
                    format!("Parameter {} not found", ident),
                ))
                .cloned()
        })
        .collect::<syn::Result<Vec<ParamInfo>>>()
}

// Get information on the return type of the function signature
fn get_info_of_const_return(sig: &Signature) -> syn::Result<Option<ReturnInfo>> {
    // Get information on all the parameters of the signature
    match &sig.output {
        syn::ReturnType::Default => Ok(None),
        syn::ReturnType::Type(_, typ) => match &**typ {
            Type::Path(path) => Ok(Some(ReturnInfo {
                param_kind: ParamKind::Owned,
                param_type: path.path.to_token_stream().to_string(),
            })),
            Type::Reference(ty_ref) => match &*ty_ref.elem {
                Type::Path(path) => Ok(Some(ReturnInfo {
                    param_kind: if ty_ref.mutability.is_some() {
                        ParamKind::MutRef
                    } else {
                        ParamKind::SharedRef
                    },
                    param_type: path.path.to_token_stream().to_string(),
                })),
                _ => Err(syn::Error::new(
                    Span::call_site(),
                    format!("Type not handled: {:?}", ty_ref),
                )),
            },
            ty => Err(syn::Error::new(
                Span::call_site(),
                format!("Type not handled: {:?}", ty),
            )),
        },
    }
}
// Transform the const_init parameters to make them mutable
// ## Example
//
// #[const_init_code_mod(target_params(self, foo))]
// fn foo(&self, foo: usize)
// becomes
// #[const_init_code_mod(target_params(self, foo))]
// fn foo(mut self: &Self, mut foo:usize)
fn set_const_init_params_to_mutable_in_sig(
    sig: &Signature,
    const_params_info: &Vec<ParamInfo>,
) -> syn::Result<Signature> {
    let mut new_sig = sig.clone();
    let ident_of_const_params = const_params_info
        .iter()
        .map(|info| info.ident.clone())
        .collect::<Vec<String>>();
    new_sig
        .inputs
        .iter_mut()
        .try_for_each(|fn_arg| match fn_arg {
            FnArg::Receiver(rec) => {
                // If "self" is a const parameter
                if ident_of_const_params.contains(&"self".to_owned()) {
                    // Change "&self" in parameter to "self"
                    rec.reference = None;
                    // Change "self" in parameter to "self: Self"
                    rec.colon_token = Some(Token![:](Span::call_site()));
                    // Change "self: Self" in parameter to "mut self: Self"
                    rec.mutability = Some(Token![mut](Span::call_site()));
                };
                Ok(())
            }
            FnArg::Typed(pat_typ) => match pat_typ.pat.as_mut() {
                syn::Pat::Ident(pat_ident) => {
                    if ident_of_const_params.contains(&pat_ident.ident.to_string()) {
                        // Change "foo: usize" in parameter to "mut foo: usize"
                        pat_ident.mutability = Some(Token![mut](Span::call_site()))
                    }
                    Ok(())
                }
                _ => Err(syn::Error::new(
                    fn_arg.span(),
                    format!(
                        "This type of parameter {:?} is not handled currently",
                        fn_arg
                    ),
                )),
            },
        })?;
    Ok(new_sig)
}

// Info on the `const_init` targets
// `const_init` targets are either parameters or return value
// of a function signature
#[derive(Debug, PartialEq, Eq, Clone)]
enum ConstInitInfo {
    Param(ParamInfo),
    Return(ReturnInfo),
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct ParamInfo {
    ident: String,
    param_kind: ParamKind,
    param_type: String,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct ReturnInfo {
    param_kind: ParamKind,
    param_type: String,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum ParamKind {
    Owned,
    SharedRef,
    MutRef,
}

impl ParamInfo {
    fn new(ident: &str, param_kind: ParamKind, param_type: &str) -> Self {
        ParamInfo {
            ident: ident.to_owned(),
            param_kind,
            param_type: param_type.to_owned(),
        }
    }
}

impl ParamInfo {
    fn from_fn_arg(param: &FnArg) -> syn::Result<Self> {
        match param {
            FnArg::Receiver(receiver) => {
                // Case where parameter is "self"
                Ok(ParamInfo {
                    ident: "self".to_owned(),
                    param_kind: ParamInfo::get_param_kind(&*receiver.ty)?,
                    param_type: ParamInfo::get_param_type(&*receiver.ty)?,
                })
            }
            FnArg::Typed(pat_type) => match *pat_type.pat {
                syn::Pat::Ident(ref pat_ident) => Ok(ParamInfo {
                    ident: pat_ident.ident.to_string(),
                    param_kind: ParamInfo::get_param_kind(&*pat_type.ty)?,
                    param_type: ParamInfo::get_param_type(&*pat_type.ty)?,
                }),

                ref pat => Err(syn::Error::new(
                    pat_type.span(),
                    format!("Unexpected pattern {:?}", pat),
                )),
            },
        }
    }

    fn get_param_kind(ty: &Type) -> syn::Result<ParamKind> {
        match ty {
            Type::Path(_) => Ok(ParamKind::Owned),
            Type::Reference(ty_ref) => match ty_ref.mutability {
                Some(_) => Ok(ParamKind::MutRef),
                None => Ok(ParamKind::SharedRef),
            },
            ty => Err(syn::Error::new(
                Span::call_site(),
                format!("Type not handled: {:?}", ty),
            )),
        }
    }

    fn get_param_type(ty: &Type) -> syn::Result<String> {
        match ty {
            Type::Path(path) => Ok(path.path.to_token_stream().to_string()),
            Type::Reference(ty_ref) => match *ty_ref.elem {
                Type::Path(ref path) => Ok(path.path.to_token_stream().to_string()),
                _ => Err(syn::Error::new(
                    Span::call_site(),
                    format!("Type not handled: {:?}", ty),
                )),
            },
            _ => Err(syn::Error::new(
                Span::call_site(),
                format!("Type not handled: {:?}", ty),
            )),
        }
    }

    // We modify the signature of the original function
    // The constant params are made mutable
    //
    // ## Example
    // #[const_init_code_modif(target_params(self, bar))]
    // fn foo(&self, bar: usize)
    //
    // becomes:
    //
    // fn foo(mut self: &Self, mut bar: usize)
}

use darling::FromMeta;

#[derive(Debug, FromMeta)]
struct ConstInitArgs {
    #[darling(default)]
    target_params: IdentList,
    #[darling(default = "set_to_false")]
    const_return: bool,
    #[darling(default = "set_to_false")]
    noop: bool,
    #[darling(default = "set_to_false")]
    replace_with_const_init: bool,
}

fn set_to_false() -> bool {
    false
}

#[derive(Debug, Default)]
struct IdentList(Vec<String>);

impl FromMeta for IdentList {
    // Get a list of ident [a,b,c,] from [const_init(target_params(a,b,c))]
    fn from_list(items: &[NestedMeta]) -> darling::Result<Self> {
        let mut idents = vec![];
        for item in items {
            match item {
                NestedMeta::Meta(syn::Meta::Path(path)) => {
                    if let Some(ident) = path.get_ident() {
                        idents.push(ident.clone().to_string())
                    } else {
                        return Err(Error::custom("Expected identifier").with_span(path));
                    }
                }
                _ => {
                    return Err(Error::custom("Expected identifier").with_span(item));
                }
            }
        }
        Ok(IdentList(idents))
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use ParamKind::*;

    #[test]
    fn params_info_are_parsed_correctly() {
        let testcases = vec![
            quote! {
                fn test_owned(self, foo: std::usize){}
            },
            quote! {
                fn test_shared_ref(&self, foo: &usize){}
            },
            quote! {
                fn test_mut_ref(&mut self, foo: &mut usize){}
            },
            quote! {
                fn test_mut(mut self, mut foo: &usize){}
            },
        ];
        let expected: Vec<Vec<ParamInfo>> = vec![
            vec![
                ParamInfo::new("self", Owned, "Self"),
                ParamInfo::new("foo", Owned, "std :: usize"),
            ],
            vec![
                ParamInfo::new("self", SharedRef, "Self"),
                ParamInfo::new("foo", SharedRef, "usize"),
            ],
            vec![
                ParamInfo::new("self", MutRef, "Self"),
                ParamInfo::new("foo", MutRef, "usize"),
            ],
            vec![
                ParamInfo::new("self", Owned, "Self"),
                ParamInfo::new("foo", SharedRef, "usize"),
            ],
        ];

        for (ts, expected) in testcases.into_iter().zip(expected) {
            let input: ItemFn = syn::parse2(ts).unwrap();
            let params_info = input
                .sig
                .inputs
                .iter()
                .map(|fn_arg| ParamInfo::from_fn_arg(fn_arg))
                .collect::<anyhow::Result<Vec<ParamInfo>, _>>()
                .unwrap();
            assert_eq!(
                params_info, expected,
                "\nGenerated:\n{:#?}\n\nExpected:\n{:#?}",
                params_info, expected
            );
        }
    }

    #[test]
    fn const_init_params_to_mutable_in_sig_is_correct() {
        let testcases = vec![
            quote! {
                fn test_owned(self, foo: std::usize){}
            },
            quote! {
                fn test_shared_ref(&self, foo: &usize){}
            },
            quote! {
                fn test_mut_ref(&mut self, foo: &mut usize){}
            },
            quote! {
                fn test_mut(mut self, mut foo: usize){}
            },
            quote! {
                fn test_self(self: Self, mut foo: usize){}
            },
        ];
        let expected = vec![
            quote! {
                fn test_owned(mut self: Self, mut foo: std::usize)
            },
            quote! {
                fn test_shared_ref(mut self: &Self, mut foo: &usize)
            },
            quote! {
                fn test_mut_ref(mut self: &mut Self, mut foo: &mut usize)
            },
            quote! {
                fn test_mut(mut self: Self, mut foo: usize)
            },
            quote! {
                fn test_self(mut self: Self, mut foo: usize)
            },
        ];

        let idents_of_const_params = vec!["self".to_owned(), "foo".to_owned()];
        for (ts, expected) in testcases.into_iter().zip(expected) {
            let input: ItemFn = syn::parse2(ts).unwrap();
            let mut sig = input.sig;
            let const_params_info =
                get_info_of_const_params(&idents_of_const_params, &sig).unwrap();
            let new_sig =
                set_const_init_params_to_mutable_in_sig(&mut sig, &const_params_info).unwrap();
            assert_eq!(new_sig.to_token_stream().to_string(), expected.to_string());
        }
    }

    #[test]
    fn code_modif_target_params_is_correct() {
        let testcases = vec![
            quote! {
                fn test_owned(self, foo: Foo){
                    drop(self);
                    drop(foo);
                }
            },
            quote! {
                fn test_shared_ref(&self, foo: &Foo){
                    println!("{:?}", self);
                    println!("{:?}", foo);
                }
            },
            quote! {
                fn test_mut_ref(&mut self, foo: &mut Foo){
                    *self = Self::new();
                    *foo = Foo::new();
                }
            },
            quote! {
                fn test_mut_owned(mut self, mut foo: Foo){
                    self = Self::new();
                    foo = Foo::new();
                    drop(self);
                    drop(foo);
                }
            },
        ];
        let expected = vec![
            quote! {
                fn test_owned(mut self: Self, mut foo: Foo) {
                    self = Self::CONST_INIT_VAR;
                    foo = Foo::CONST_INIT_VAR;
                    drop(self);
                    drop(foo);
                }
            },
            quote! {
                fn test_shared_ref(mut self: &Self, mut foo: &Foo){
                    self = &Self::CONST_INIT_VAR;
                    foo = &Foo::CONST_INIT_VAR;
                    println!("{:?}", self);
                    println!("{:?}", foo);
                }
            },
            quote! {
                fn test_mut_ref(mut self: &mut Self, mut foo: &mut Foo){
                    *self =  Self::CONST_INIT_VAR;
                    *foo =  Foo::CONST_INIT_VAR;
                    *self = Self::new();
                    *foo = Foo::new();
                }
            },
            quote! {
                fn test_mut_owned(mut self: Self, mut foo: Foo){
                    self =  Self::CONST_INIT_VAR;
                    foo =  Foo::CONST_INIT_VAR;
                    self = Self::new();
                    foo = Foo::new();
                    drop(self);
                    drop(foo);
                }
            },
        ];

        let idents_of_const_params = vec!["foo".to_owned(), "self".to_owned()];
        for (ts, expected) in testcases.into_iter().zip(expected) {
            let item_fn: ItemFn = syn::parse2(ts).unwrap();
            let expanded =
                init_target_params_with_const_init(&item_fn, &idents_of_const_params).unwrap();
            assert_eq!(expanded.to_string(), expected.to_string());
        }
    }

    #[test]
    fn code_modif_replace_with_noop_is_correct() {
        let testcases = vec![
            quote! {
                fn test_owned(self, foo: Foo){
                    drop(self);
                    drop(foo);
                }
            },
            quote! {
                const fn test_owned(self, foo: Foo){
                    drop(self);
                    drop(foo);
                }
            },
            quote! {
                fn test_shared_ref(&self, foo: &Foo) -> Self {
                    Self::new()
                }
            },
        ];
        let expected = vec![
            Ok(quote! {
                const fn test_owned(self, foo: Foo){
                }
            }),
            Ok(quote! {
                const fn test_owned(self, foo: Foo) {
                }
            }),
            Err(()),
        ];

        for (ts, expected) in testcases.into_iter().zip(expected) {
            let item_fn: ItemFn = syn::parse2(ts).unwrap();
            let expanded = replace_whole_fn_with_noop(&item_fn);
            match (expanded, expected) {
                (Ok(expanded), Ok(expected)) => {
                    assert_eq!(expanded.to_string(), expected.to_string())
                }
                (Err(_), Err(_)) => assert!(true),
                _ => assert!(false),
            }
        }
    }

    #[test]
    fn code_modif_replace_whole_block_with_const_init_is_correct() {
        let testcases = vec![
            quote! {
                fn test_owned(self, foo: Foo) -> Self {
                    drop(self);
                    drop(foo);
                }
            },
            quote! {
                const fn test_const(&self, foo: &Foo) -> Self {
                    drop(self);
                    drop(foo);
                }
            },
            quote! {
                fn test_shared_ref(&self, foo: &Foo) -> &Self {
                    self
                }
            },
        ];
        let expected = vec![
            Ok(quote! {
                const fn test_owned(self, foo: Foo) -> Self {
                    Self::const_init()
                }
            }),
            Ok(quote! {
                const fn test_const(&self, foo: &Foo) -> Self {
                    Self::const_init()
                }
            }),
            Err(()),
        ];

        for (ts, expected) in testcases.into_iter().zip(expected) {
            let item_fn: ItemFn = syn::parse2(ts).unwrap();
            let expanded = replace_whole_fn_with_const_init(&item_fn);
            match (expanded, expected) {
                (Ok(expanded), Ok(expected)) => {
                    assert_eq!(expanded.to_string(), expected.to_string())
                }
                (Err(_), Err(_)) => assert!(true),
                _ => assert!(false),
            }
        }
    }
}
