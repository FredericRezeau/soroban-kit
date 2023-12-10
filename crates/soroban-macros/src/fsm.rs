/*
    Date: 2023
    Author: Fred Kyung-jin Rezeau <fred@litemint.com>
    Copyright (c) 2023 Litemint LLC

    MIT License
*/

extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, AttributeArgs, Ident, ItemFn, Lit, Meta, NestedMeta};

#[allow(unused_imports)]
use soroban_tools::fsm::StorageType;

pub fn state_machine(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as AttributeArgs);
    let mut input_fn = parse_macro_input!(item as ItemFn);
    let mut state_path = [None, None, None];
    let mut region_path = [None, None, None];
    let mut storage_type = None;
    let mut is_transition_handler = false;

    // Parse macro attributes:
    // state: StatePath := EnumName ":" VariantName [":" TupleValue]
    // region: RegionPath := EnumName ":" VariantName [":" TupleValue]
    // transition: true | false
    // storage: instance | persistent | temporary
    for arg in args {
        match arg {
            NestedMeta::Meta(Meta::NameValue(nv)) => match nv.path.get_ident() {
                Some(ident) if ident == "state" => state_path = parse_path(&nv.lit),
                Some(ident) if ident == "region" => region_path = parse_path(&nv.lit),
                Some(ident) if ident == "handler" => {
                    is_transition_handler = matches!(nv.lit, Lit::Bool(b) if b.value)
                }
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
    let state_machine_body = {
        match (state_path[2].clone(), region_path[2].clone()) {
            (None, None) => match region_path[0].clone() {
                Some(_) => {
                    quote! {
                        soroban_tools::impl_state_machine!(
                            self,
                            env,
                            soroban_tools::fsm::StorageType::#storage_type_ident,
                            #is_transition_handler,
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
                            #is_transition_handler,
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
                        #is_transition_handler,
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
                            #is_transition_handler,
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
                            #is_transition_handler,
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
                        #is_transition_handler,
                        #state_enum, #state_variant, ( #state_tuple_value_expr.clone() ),
                        #region_enum, #region_variant, ( #region_tuple_value_expr.clone() )
                    );
                }
            }
        }
    };

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

fn parse_path(attr: &Lit) -> [Option<String>; 3] {
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

fn get_storage_type(storage_type_str: &Option<String>) -> Ident {
    match storage_type_str.as_deref() {
        Some("persistent") => format_ident!("Persistent"),
        Some("temporary") => format_ident!("Temporary"),
        _ => format_ident!("Instance"),
    }
}
