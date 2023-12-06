/*
    Date: 2023
    Author: Fred Kyung-jin Rezeau <fred@litemint.com>
    Copyright (c) 2023 Litemint LLC

    MIT License
*/

//! A collection of procedural macros designed to streamline development for Soroban
//! smart contracts.

//! ##Storage
//! 
//! The `storage` and `key_constraint` macros can be used to implement type safety
//! for storage operations while also enforcing type rules at compile time to bind
//! the Soroban storage, data types and keys.
//!
//! For performance, the code generates a minimal wrapper that handles key and data operations
//! without duplication, leveraging Rust lifetimes for safe borrowing.
//!
//! # Background
//!
//! When dealing with the Soroban storage, repetitive boilerplate code is typically required
//! for encapsulation and type safety over generic storage functions.
//! e.g.
//!  fn set_user_data(key: &userKey, data: &UserData) // Persists user data in storage.
//!  fn get_user_data(key: &userKey) -> UserData // Retrieves user data from storage.
//!
//! These macros streamline this process by automatically generating the boilerplate
//! code, enforcing type rules at compile time, binding the storage with custom data types and
//! optionally, applying Trait constraints on storage keys with `key_constraint`.
//!
//! # Usage
#[cfg_attr(
    feature = "include_doctests",
    doc = r#"
//!   ```rust,ignore
//! 
//!      use soroban_macros::{storage, key_constraint};
//! 
//!      // Key constraints are compile-time restrictions ensuring
//!      // that only specific key types can be used with the storage.
//!      #[key_constraint(AdminKeyConstraint)]
//!      #[contracttype]
//!      pub enum Key {
//!          User(Address),
//!      }
//! 
//!      // Use the `storage` macro to implement the desired storage for any custom contract type.
//!      // Example: Implementing Soroban Instance storage for CustomType
//!      #[storage(Instance)]
//!      // #[storage(Instance, AdminKeyConstraint)]  // Optionally apply the constraint.
//!      #[contracttype]
//!      pub struct CustomType {
//!          pub token: Address,
//!      }
//! 
//!      // You now have access to type-safe operations encapsulating
//!      // the instance storage, binding with CustomType.
//! 
//!      let key = Key::User(Address::random(&env)); 
//!      // Example: Set data.
//!      storage::set::<Key, CustomType>(&env, &key, &CustomType { token: Address::random(&env) });
//!      // Example: Get data.
//!      storage::get::<Key, CustomType>(&env, &key);
//!      // Example: Get data with error tolerance.
//!      storage::get_or_else::<Key, CustomType, _, _>(&env, &key, |opt| opt.unwrap_or_else(|| default_value()));
//!      // Example: Checking that data exists.
//!      storage::has::<Key, CustomType>(&env, &key);
//!      // Example: Bumping data lifetime.
//!      storage::bump::<Key, CustomType>(&env, &key, 1, 1);
//!      // Example: Deleting the data.
//!      storage::remove::<Key, CustomType>(&env, &key);
//!   ```
"#
)]

extern crate proc_macro;

#[allow(unused_imports)]
use proc_macro::TokenStream;

/// Storage procedural macros implementation.
#[cfg(feature = "storage")]
mod storage;

#[cfg(feature = "storage")]
#[proc_macro_attribute]
pub fn storage(attr: TokenStream, item: TokenStream) -> TokenStream {
    storage::storage(attr, item)
}

#[cfg(feature = "storage")]
#[proc_macro_attribute]
pub fn key_constraint(attr: TokenStream, item: TokenStream) -> TokenStream {
    storage::key_constraint(attr, item)
}

