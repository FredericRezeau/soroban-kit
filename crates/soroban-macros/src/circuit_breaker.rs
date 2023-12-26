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

use crate::fsm::{get_storage_type, parse_path};

pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ty = input.ident;
    let expanded = quote! {
        impl<K> soroban_tools::fsm::TransitionHandler<K, bool> for #ty
        where
            K: Clone + soroban_sdk::IntoVal<soroban_sdk::Env, soroban_sdk::Val> + soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>,
        {
            fn on_guard(&self, env: &soroban_sdk::Env, state_machine: &soroban_tools::fsm::StateMachine<K, bool>) {
            }
            fn on_effect(&self, _env: &soroban_sdk::Env, _state_machine: &soroban_tools::fsm::StateMachine<K, bool>) {
            }
        }
    };
    expanded.into()
}

pub fn when(attr: TokenStream, input: TokenStream, opened: bool) -> TokenStream {
    let args = parse_macro_input!(attr as AttributeArgs);
    let mut input_fn = parse_macro_input!(input as ItemFn);

    let (region_path, storage_type, is_trigger) = parse_attributes(&args);

    // XOR trigger ^ state.
    let state_variant = format_ident!("{}", opened != is_trigger);

    let region_enum = region_path[0]
        .as_ref()
        .map(|e| format_ident!("{}", e))
        .unwrap_or_else(|| format_ident!("_"));
    let region_variant = region_path[1]
        .as_ref()
        .map(|e| format_ident!("{}", e))
        .unwrap_or_else(|| format_ident!("_"));
    let region_tuple_value_expr = region_path[2]
        .as_ref()
        .map(|p| syn::parse_str::<syn::Expr>(p).unwrap())
        .unwrap_or_else(|| syn::parse_str::<syn::Expr>("()").unwrap());

    let storage_type_ident = get_storage_type(&storage_type);

    let state_machine_body = impl_circuit_breaker_state_machine(
        &is_trigger,
        &region_path,
        &state_variant,
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

fn impl_circuit_breaker_state_machine(
    is_trigger: &bool,
    region_path: &[Option<String>; 3],
    state_variant: &Ident,
    region_enum: &Ident,
    region_variant: &Ident,
    region_tuple_value_expr: &syn::Expr,
    storage_type_ident: &Ident,
) -> proc_macro2::TokenStream {
    {
        match region_path[2].clone() {
            None => match region_path[0].clone() {
                None => {
                    quote! {
                        soroban_tools::impl_circuit_breaker_state_machine!(
                            self,
                            env,
                            #is_trigger,
                            soroban_tools::fsm::StorageType::#storage_type_ident, bool, #state_variant
                        );
                    }
                }
                Some(_) => {
                    quote! {
                        soroban_tools::impl_circuit_breaker_state_machine!(
                            self,
                            env,
                            #is_trigger,
                            soroban_tools::fsm::StorageType::#storage_type_ident, bool, #state_variant, #region_enum, #region_variant, ()
                        );
                    }
                }
            },
            Some(_) => {
                quote! {
                    soroban_tools::impl_circuit_breaker_state_machine!(
                        self,
                        env,
                        #is_trigger,
                        soroban_tools::fsm::StorageType::#storage_type_ident, bool, #state_variant, #region_enum, #region_variant, ( #region_tuple_value_expr.clone() )
                    );
                }
            }
        }
    }
}

pub fn parse_attributes(args: &AttributeArgs) -> ([Option<String>; 3], Option<String>, bool) {
    let mut region_path = [None, None, None];
    let mut storage_type = None;
    let mut is_trigger = false;

    for arg in args {
        match arg {
            NestedMeta::Meta(Meta::NameValue(nv)) => match nv.path.get_ident() {
                Some(ident) if ident == "region" => region_path = parse_path(&nv.lit),
                Some(ident) if ident == "storage" => {
                    if let Lit::Str(lit_str) = &nv.lit {
                        storage_type = Some(lit_str.value());
                    }
                }
                Some(ident) if ident == "trigger" => {
                    if let Lit::Bool(lit_bool) = &nv.lit {
                        is_trigger = lit_bool.value;
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }

    (region_path, storage_type, is_trigger)
}
