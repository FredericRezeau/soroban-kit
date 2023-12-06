#![no_std]

/// The `storage` module exports the `impl_storage!` macro which can be used to implement
/// type safety for storage operations while also enforcing type rules at compile time
/// to bind the Soroban storage, data types and keys.
///
/// For performance, the code generates a minimal wrapper that handles key and data operations
/// without duplication, leveraging Rust lifetimes for safe borrowing.
///
/// # Background
///
/// When dealing with the Soroban storage, repetitive boilerplate code is typically required
/// for encapsulation and type safety over generic storage functions.
/// e.g.
///  fn set_user_data(key: &userKey, data: &UserData) // Persists user data in storage.
///  fn get_user_data(key: &userKey) -> UserData // Retrieves user data from storage.
///
/// The `impl_storage!` macro streamlines this process by automatically generating the boilerplate
/// code, enforcing type rules at compile time, binding the storage with custom data types and
/// optionally, applying Trait constraints to storage keys.
///
/// # Usage
#[cfg_attr(
    feature = "include_doctests",
    doc = r#"
///   ```rust,ignore
/// 
///      use soroban_tools::{impl_storage, impl_key_constraint, storage};
/// 
///      #[contracttype]
///      pub enum Key {
///          User(Address),
///      }
///
///      #[contracttype]
///      pub struct CustomType {
///          pub token: Address,
///      }
/// 
///      // Use the `impl_storage!` macro to implement the desired storage
///      // for any custom contract type:
/// 
///      // Example: Implementing Soroban Instance storage for CustomType
///      impl_storage!(Instance, CustomType);
/// 
///      // (Optional) Key constraints are compile-time restrictions ensuring
///      // that only specific key types can be used with the storage.
///      // Example:
///      //     impl_key_constraint!(Key, ChooseAnyConstraintName); 
///      //     impl_storage!(Instance, CustomType, ChooseAnyConstraintName); 
/// 
///      // You now have access to type-safe operations encapsulating
///      // the instance storage, binding with CustomType.
/// 
///      let key = Key::User(Address::random(&env)); 
///      // Example: Set data.
///      storage::set::<Key, CustomType>(&env, &key, &CustomType { token: Address::random(&env) });
///      // Example: Get data.
///      storage::get::<Key, CustomType>(&env, &key);
///      // Example: Get data with error tolerance.
///      storage::get_or_else::<Key, CustomType, _, _>(&env, &key, |opt| opt.unwrap_or_else(|| default_value()));
///      // Example: Checking that data exists.
///      storage::has::<Key, CustomType>(&env, &key);
///      // Example: Bumping data lifetime.
///      storage::bump::<Key, CustomType>(&env, &key, 1, 1);
///      // Example: Deleting the data.
///      storage::remove::<Key, CustomType>(&env, &key);
///   ```
"#
)]
#[cfg(feature = "storage")]
pub mod storage;

#[cfg(any(test, feature = "mock-storage"))]
pub mod test_utils;
