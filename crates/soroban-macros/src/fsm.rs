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
use quote::{format_ident, quote};
use syn::{parse_macro_input, AttributeArgs, DeriveInput, Ident, ItemFn, Lit, Meta, NestedMeta};

#[allow(unused_imports)]
use soroban_tools::fsm::StorageType;

pub fn transition_handler_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ty = input.ident;
    let expanded = quote! {
        impl<K, V> soroban_tools::fsm::TransitionHandler<K, V> for #ty
        where
            K: Clone
                + soroban_sdk::IntoVal<soroban_sdk::Env, soroban_sdk::Val>
                + soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>,
            V: Clone
                + soroban_sdk::IntoVal<soroban_sdk::Env, soroban_sdk::Val>
                + soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>,
        {
            fn on_guard(&self, _env: &soroban_sdk::Env, _state_machine: &soroban_tools::fsm::StateMachine<K, V>) {
            }

            fn on_effect(&self, _env: &soroban_sdk::Env, _state_machine: &soroban_tools::fsm::StateMachine<K, V>) {
            }
        }
    };
    expanded.into()
}

pub fn state_machine(attr: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as AttributeArgs);
    let mut input_fn = parse_macro_input!(input as ItemFn);

    let (state_path, region_path, storage_type) = parse_attributes(&args);

    // Convert params and call soroban_tools::impl_state_machine! macro.
    let state_enum = state_path[0]
        .as_ref()
        .map(|e| format_ident!("{}", e))
        .unwrap_or_else(|| format_ident!(""));
    let state_variant = state_path[1]
        .as_ref()
        .map(|e| format_ident!("{}", e))
        .unwrap_or_else(|| format_ident!(""));
    let state_tuple_value_expr = state_path[2]
        .as_ref()
        .map(|p| syn::parse_str::<syn::Expr>(p).unwrap())
        .unwrap_or_else(|| syn::parse_str::<syn::Expr>("()").unwrap());

    let region_enum = region_path[0]
        .as_ref()
        .map(|e| format_ident!("{}", e))
        .unwrap_or_else(|| format_ident!("StateMachineRegion"));
    let region_variant = region_path[1]
        .as_ref()
        .map(|e| format_ident!("{}", e))
        .unwrap_or_else(|| format_ident!("Closed"));
    let region_tuple_value_expr = region_path[2]
        .as_ref()
        .map(|p| syn::parse_str::<syn::Expr>(p).unwrap())
        .unwrap_or_else(|| syn::parse_str::<syn::Expr>("()").unwrap());

    let storage_type_ident = get_storage_type(&storage_type);

    let state_machine_body = impl_state_machine(
        &state_path,
        &region_path,
        &state_enum,
        &state_variant,
        &state_tuple_value_expr,
        &region_enum,
        &region_variant,
        &region_tuple_value_expr,
        &storage_type_ident,
    );

    // Prepend state machine code to function body.
    let original_body = input_fn.block;
    input_fn.block = syn::parse(
        quote!({
            #state_machine_body
            #original_body
        })
        .into(),
    )
    .expect("Failed to parse body");
    TokenStream::from(quote!(#input_fn))
}

pub fn impl_state_machine(
    state_path: &[Option<String>; 3],
    region_path: &[Option<String>; 3],
    state_enum: &Ident,
    state_variant: &Ident,
    state_tuple_value_expr: &syn::Expr,
    region_enum: &Ident,
    region_variant: &Ident,
    region_tuple_value_expr: &syn::Expr,
    storage_type_ident: &Ident,
) -> proc_macro2::TokenStream {
    {
        match (state_path[2].clone(), region_path[2].clone()) {
            (None, None) => match region_path[0].clone() {
                Some(_) => {
                    quote! {
                        soroban_tools::impl_state_machine!(
                            self,
                            env,
                            soroban_tools::fsm::StorageType::#storage_type_ident,
                            #state_enum, #state_variant, (),
                            #region_enum, #region_variant, ()
                        );
                    }
                }
                None => {
                    quote! {
                        soroban_tools::impl_state_machine!(
                            self,
                            env,
                            soroban_tools::fsm::StorageType::#storage_type_ident,
                            #state_enum, #state_variant
                        );
                    }
                }
            },
            (None, Some(_)) => {
                quote! {
                    soroban_tools::impl_state_machine!(
                        self,
                        env,
                        soroban_tools::fsm::StorageType::#storage_type_ident,
                        #state_enum, #state_variant, (),
                        #region_enum, #region_variant, ( #region_tuple_value_expr.clone() )
                    );
                }
            }
            (Some(_), None) => match region_path[0].clone() {
                Some(_) => {
                    quote! {
                        soroban_tools::impl_state_machine!(
                            self,
                            env,
                            soroban_tools::fsm::StorageType::#storage_type_ident,
                            #state_enum, #state_variant, ( #state_tuple_value_expr.clone() ),
                            #region_enum, #region_variant, ()
                        );
                    }
                }
                None => {
                    quote! {
                        soroban_tools::impl_state_machine!(
                            self,
                            env,
                            soroban_tools::fsm::StorageType::#storage_type_ident,
                            #state_enum, #state_variant, ( #state_tuple_value_expr.clone() )
                        );
                    }
                }
            },
            (Some(_), Some(_)) => {
                quote! {
                    soroban_tools::impl_state_machine!(
                        self,
                        env,
                        soroban_tools::fsm::StorageType::#storage_type_ident,
                        #state_enum, #state_variant, ( #state_tuple_value_expr.clone() ),
                        #region_enum, #region_variant, ( #region_tuple_value_expr.clone() )
                    );
                }
            }
        }
    }
}

pub fn parse_attributes(
    args: &AttributeArgs,
) -> ([Option<String>; 3], [Option<String>; 3], Option<String>) {
    let mut state_path = [None, None, None];
    let mut region_path = [None, None, None];
    let mut storage_type = None;

    for arg in args {
        match arg {
            NestedMeta::Meta(Meta::NameValue(nv)) => match nv.path.get_ident() {
                Some(ident) if ident == "state" => state_path = parse_path(&nv.lit),
                Some(ident) if ident == "region" => region_path = parse_path(&nv.lit),
                Some(ident) if ident == "storage" => {
                    if let Lit::Str(lit_str) = &nv.lit {
                        storage_type = Some(lit_str.value());
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }

    (state_path, region_path, storage_type)
}

pub fn parse_path(attr: &Lit) -> [Option<String>; 3] {
    if let Lit::Str(lit) = attr {
        let value = lit.value();
        let parts: Vec<&str> = value.split(':').collect();
        let mut array = [None, None, None];
        for (i, part) in parts.iter().enumerate() {
            if i < 3 {
                array[i] = Some(part.to_string());
            } else {
                break;
            }
        }
        array
    } else {
        [None, None, None]
    }
}

pub fn get_storage_type(storage_type_str: &Option<String>) -> Ident {
    match storage_type_str.as_deref() {
        Some("persistent") => format_ident!("Persistent"),
        Some("temporary") => format_ident!("Temporary"),
        _ => format_ident!("Instance"),
    }
}
