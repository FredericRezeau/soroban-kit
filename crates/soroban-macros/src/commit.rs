/*
    Date: 2023
    Author: Fred Kyung-jin Rezeau <fred@litemint.com>
    Copyright (c) 2023 Litemint LLC

    MIT License
*/

extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, AttributeArgs, ItemFn, Lit, Meta, NestedMeta};

pub fn commit(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as AttributeArgs);
    let mut input_fn = parse_macro_input!(item as ItemFn);
    let mut hash_name = "hash".to_string();
    let mut storage_name = "instance".to_string();

    for arg in args {
        match arg {
            NestedMeta::Meta(Meta::NameValue(nv)) => match nv.path.get_ident() {
                Some(ident) if ident == "storage" => {
                    if let Lit::Str(lit_str) = &nv.lit {
                        storage_name = lit_str.value()
                    }
                }
                Some(ident) if ident == "hash" => {
                    if let Lit::Str(lit_str) = &nv.lit {
                        hash_name = lit_str.value();
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }

    let hash_expr = Some(syn::parse_str::<syn::Expr>(&hash_name).expect("Invalid hash parameter"));
    let commit_body = {
        let storage = match storage_name.as_str() {
            "Persistent" => quote! { env.storage().persistent() },
            "Temporary" => quote! { env.storage().temporary() },
            _ => quote! { env.storage().instance() },
        };
        quote! {
            if !#storage.has::<BytesN<32>>(&#hash_expr) {
                #storage.set::<BytesN<32>, i32>(&#hash_expr, &0i32);
            }
            else {
                panic!("Failed to commit");
            }
        }
    };

    // Prepend commit code to function body.
    let original_body = input_fn.block;
    input_fn.block = syn::parse(
        quote!({
            #commit_body
            #original_body
        })
        .into(),
    )
    .expect("Failed to parse body");

    TokenStream::from(quote!(#input_fn))
}

pub fn reveal(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as AttributeArgs);
    let mut input_fn = parse_macro_input!(item as ItemFn);
    let mut data_name = "data".to_string();
    let mut hash_func_name = "sha256".to_string();
    let mut storage_name = "instance".to_string();
    let mut remove = true;

    for arg in args {
        match arg {
            NestedMeta::Meta(Meta::NameValue(nv)) => match nv.path.get_ident() {
                Some(ident) if ident == "storage" => {
                    if let Lit::Str(lit_str) = &nv.lit {
                        storage_name = lit_str.value();
                    }
                }
                Some(ident) if ident == "data" => {
                    if let Lit::Str(lit_str) = &nv.lit {
                        data_name = lit_str.value();
                    }
                }
                Some(ident) if ident == "hash_func" => {
                    if let Lit::Str(lit_str) = &nv.lit {
                        hash_func_name = lit_str.value();
                    }
                }
                Some(ident) if ident == "clear_commit" => {
                    remove = matches!(nv.lit, Lit::Bool(b) if b.value)
                }
                _ => {}
            },
            _ => {}
        }
    }

    let data_expr = syn::parse_str::<syn::Expr>(&data_name).expect("Invalid data parameter");
    let reveal_body = {
        let storage = match storage_name.as_str() {
            "persistent" => quote! { env.storage().persistent() },
            "temporary" => quote! { env.storage().temporary() },
            _ => quote! { env.storage().instance() },
        };
        quote! {
            let computed_hash = match #hash_func_name {
                "sha256" => env.crypto().sha256(&#data_expr),
                "keccak256" => env.crypto().keccak256(&#data_expr),
                _ => panic!("Invalid hash function")
            };

            if !#storage.has::<BytesN<32>>(&computed_hash) {
                panic!("Failed to reveal");
            }
            else {
                if #remove {
                    #storage.remove::<BytesN<32>>(&computed_hash);
                }
            }
        }
    };

    // Prepend reveal code to function body.
    let original_body = input_fn.block;
    input_fn.block = syn::parse(
        quote!({
            #reveal_body
            #original_body
        })
        .into(),
    )
    .expect("Failed to parse body");

    TokenStream::from(quote!(#input_fn))
}
