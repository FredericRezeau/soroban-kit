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
use syn::DeriveInput;
use syn::{parse_macro_input, AttributeArgs, ItemStruct};

#[allow(unused_imports)]
use soroban_tools::impl_key_constraint;
#[allow(unused_imports)]
use soroban_tools::impl_storage;

pub fn storage(attr: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as AttributeArgs);
    let input = parse_macro_input!(input as ItemStruct);
    let storage_type = match args.first() {
        Some(syn::NestedMeta::Meta(syn::Meta::Path(p))) => quote! { #p },
        _ => unimplemented!(),
    };

    let trait_ident = args.get(1).and_then(|arg| {
        if let syn::NestedMeta::Meta(syn::Meta::Path(p)) = arg {
            Some(quote! { , #p })
        } else {
            None
        }
    });

    // Invoke the impl_storage! macro (soroban-tools).
    let struct_ident = &input.ident;
    let expanded = quote! {
        soroban_tools::impl_storage!(#storage_type, #struct_ident #trait_ident);
    };

    // Return original struct combined with storage impl.
    let output = quote! {
        #input
        #expanded
    };
    output.into()
}

pub fn key_constraint(attr: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as AttributeArgs);
    let input = parse_macro_input!(input as DeriveInput);

    assert_eq!(args.len(), 1);

    let key_trait = match &args[0] {
        syn::NestedMeta::Meta(syn::Meta::Path(path)) => quote! { #path },
        _ => panic!("Expected a trait"),
    };

    let type_ident = &input.ident;

    // Invoke the impl_key_constraint! macro (soroban-tools).
    let expanded = quote! {
        soroban_tools::impl_key_constraint!(#type_ident, #key_trait);
    };

    // Return original struct combined with storage impl.
    let output = quote! {
        #input
        #expanded
    };
    output.into()
}