/*
    Date: 2023
    Author: Fred Kyung-jin Rezeau <fred@litemint.com>
    Copyright (c) 2023 Litemint LLC

    MIT License
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

pub fn storage(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as AttributeArgs);
    let input = parse_macro_input!(item as ItemStruct);
    let storage_type = match args.first() {
        Some(syn::NestedMeta::Meta(syn::Meta::Path(p))) => quote! { #p },
        _ => panic!("Expected a storage type (Instance, Persistent, Temporary)."),
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

pub fn key_constraint(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as AttributeArgs);
    let input = parse_macro_input!(item as DeriveInput);
    if args.len() != 1 {
        panic!("Expected one argument: trait.");
    }

    let key_trait = match &args[0] {
        syn::NestedMeta::Meta(syn::Meta::Path(path)) => quote! { #path },
        _ => panic!("Expected a trait name for the key_constraint macro"),
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