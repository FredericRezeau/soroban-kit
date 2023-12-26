/*
    Copyright (c) 2023-2024 Frederic Kyung-jin Rezeau (오경진 吳景振)

    This file is part of soroban-kit.

    Licensed under the MIT License, this software is provided "AS IS",
    no liability assumed. For details, see the LICENSE file in the
    root directory.

    Author: Fred Kyung-jin Rezeau <fred@litemint.com>
*/

extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, AttributeArgs, ItemFn, Lit, Meta, NestedMeta};

pub fn commit(attr: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as AttributeArgs);
    let mut input_fn = parse_macro_input!(input as ItemFn);
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
            assert!(!#storage.has::<BytesN<32>>(&#hash_expr));
            #storage.set::<BytesN<32>, i32>(&#hash_expr, &0i32);
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

pub fn reveal(attr: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as AttributeArgs);
    let mut input_fn = parse_macro_input!(input as ItemFn);
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
                _ => unimplemented!()
            };

            assert!(#storage.has::<BytesN<32>>(&computed_hash));
            if #remove {
                #storage.remove::<BytesN<32>>(&computed_hash);
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
