/*
    Date: 2023
    Author: Fred Kyung-jin Rezeau <fred@litemint.com>
    Copyright (c) 2023 Litemint LLC

    MIT License
*/

extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput};

// Broker oracle interface for cross-contract calls.
fn oracle_broker_trait<K: quote::ToTokens, V: quote::ToTokens>(
    k_ty: K,
    v_ty: V,
) -> proc_macro2::TokenStream {
    quote! {
        fn subscribe(env: Env, topic: #k_ty, envelope: soroban_kit::oracle::Envelope) -> Option<#v_ty>;
        fn publish(env: Env, topic: #k_ty, publisher: Address, data: #v_ty);
    }
}

// Subscriber oracle interface for cross-contract calls.
fn oracle_subscriber_trait<K: quote::ToTokens, V: quote::ToTokens>(
    k_ty: K,
    v_ty: V,
) -> proc_macro2::TokenStream {
    quote! {
        fn request(env: Env, topic: #k_ty, subscriber: Address, broker: Address) -> Option<#v_ty>;
        fn receive(env: Env, topic: #k_ty, envelope: soroban_kit::oracle::Envelope, data: #v_ty);
    }
}


pub fn oracle_subscriber_attribute(args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ty = input.ident.clone();
    let args = parse_macro_input!(args as syn::AttributeArgs);
    assert_eq!(args.len(), 2);
    let (k_ty, v_ty) = match args.as_slice() {
        [syn::NestedMeta::Meta(syn::Meta::Path(k_path)), syn::NestedMeta::Meta(syn::Meta::Path(v_path))] => {
            (quote!(#k_path), quote!(#v_path))
        }
        _ => panic!(),
    };
    let subscriber_trait = format_ident!("OracleSubscriberFor{}", ty);
    let broker_trait = format_ident!("OracleBrokerFor{}", ty);
    let broker_trait_client = format_ident!("OracleBrokerFor{}Client", ty);
    let broker_trait_client_name = broker_trait_client.to_string();
    let broker_trait_methods = oracle_broker_trait(k_ty.clone(), v_ty.clone());
    let subscriber_trait_methods = oracle_subscriber_trait(k_ty.clone(), v_ty.clone());
    let expanded = quote! {
        #input
        pub trait #subscriber_trait {
            #subscriber_trait_methods
        }
        #[soroban_sdk::contractclient(name = #broker_trait_client_name)]
        pub trait #broker_trait {
            #broker_trait_methods
        }
        #[contractimpl]
        impl #subscriber_trait for #ty {
            fn request(
                env: Env, topic: #k_ty, subscriber: Address, broker: Address
            ) -> Option<#v_ty> {
                assert_ne!(env.current_contract_address(), subscriber);
                assert_ne!(env.current_contract_address(), broker);
                let envelope = soroban_kit::oracle::Envelope {
                    subscriber,
                    broker: broker.clone(),
                    router: env.current_contract_address(),
                };
                <#ty as soroban_kit::oracle::Events<#k_ty, #v_ty>>::on_request(
                    &env, &topic, &envelope,
                );
                if let Some(data) = #broker_trait_client::new(&env, &broker).subscribe(&topic, &envelope) {
                    <#ty as soroban_kit::oracle::Events<#k_ty, #v_ty>>::on_sync_receive
                        (&env, &topic, &envelope, &data);
                    Some(data)
                } else {
                    None
                }
            }
            fn receive(
                env: Env,
                topic: #k_ty,
                envelope: soroban_kit::oracle::Envelope,
                data: #v_ty
            ) {
                assert_ne!(env.current_contract_address(), envelope.subscriber);
                assert_ne!(env.current_contract_address(), envelope.broker);
                <#ty as soroban_kit::oracle::Events<#k_ty, #v_ty>>::on_async_receive(
                    &env, &topic, &envelope, &data
                );
            }
        }
    };
    expanded.into()
}

pub fn oracle_broker_attribute(args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ty = input.ident.clone();
    let args = parse_macro_input!(args as syn::AttributeArgs);
    assert_eq!(args.len(), 2);
    let (k_ty, v_ty) = match args.as_slice() {
        [syn::NestedMeta::Meta(syn::Meta::Path(k_path)), syn::NestedMeta::Meta(syn::Meta::Path(v_path))] => {
            (quote!(#k_path), quote!(#v_path))
        }
        _ => panic!(),
    };
    let broker_trait = format_ident!("OracleBrokerFor{}", ty);
    let subscriber_trait = format_ident!("OracleSubscriberFor{}", ty);
    let subscriber_trait_client = format_ident!("OracleSubscriberFor{}Client", ty);
    let subscriber_trait_client_name = subscriber_trait_client.to_string();
    let broker_trait_methods = oracle_broker_trait(k_ty.clone(), v_ty.clone());
    let subscriber_trait_methods = oracle_subscriber_trait(k_ty.clone(), v_ty.clone());
    let expanded = quote! {
        #input
        pub trait #broker_trait {
            #broker_trait_methods
        }
        #[soroban_sdk::contractclient(name = #subscriber_trait_client_name)]
        pub trait #subscriber_trait {
            #subscriber_trait_methods
        }
        #[contractimpl]
        impl #broker_trait for #ty {
            fn subscribe(
                env: Env, topic: #k_ty, envelope: soroban_kit::oracle::Envelope
            ) -> Option<#v_ty> {
                assert_ne!(env.current_contract_address(), envelope.subscriber);
                assert_ne!(env.current_contract_address(), envelope.router);
                <#ty as soroban_kit::oracle::Events<#k_ty, #v_ty>>::on_subscribe
                    (&env, &topic, &envelope)
            }
            fn publish(
                env: Env, topic: #k_ty, publisher: Address, data: #v_ty
            ) {
                let envelopes = <#ty as soroban_kit::oracle::Events<#k_ty, #v_ty>>::on_publish(
                    &env, &topic, &data, &publisher,
                );
                envelopes.iter().for_each(|envelope| {
                    #subscriber_trait_client::new(&env, &envelope.router).receive
                        (&topic, &envelope, &data);
                });
            }
        }
    };
    expanded.into()
}
