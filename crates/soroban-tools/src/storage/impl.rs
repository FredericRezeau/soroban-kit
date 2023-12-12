/*
    Date: 2023
    Author: Fred Kyung-jin Rezeau <fred@litemint.com>
    Copyright (c) 2023 Litemint LLC

    MIT License
*/

#[cfg(not(feature = "mock-storage"))]
use soroban_sdk::storage::{Instance, Persistent, Temporary};

#[cfg(feature = "mock-storage")]
pub use crate::test_utils::{
    with_instance_storage, with_persistent_storage, with_temporary_storage,
};

use soroban_sdk::{Env, IntoVal, TryFromVal, Val};

use core::marker::PhantomData;

/// Execute the provided closure with instance storage.
#[inline]
#[cfg(not(feature = "mock-storage"))]
pub fn with_instance_storage<F, T>(env: &Env, f: F) -> T
where
    F: FnOnce(&Instance) -> T,
{
    f(&env.storage().instance())
}

/// Execute the provided closure with persistent storage.
#[inline]
#[cfg(not(feature = "mock-storage"))]
pub fn with_persistent_storage<F, T>(env: &Env, f: F) -> T
where
    F: FnOnce(&Persistent) -> T,
{
    f(&env.storage().persistent())
}

/// Execute the provided closure with temporary storage.
#[inline]
#[cfg(not(feature = "mock-storage"))]
pub fn with_temporary_storage<F, T>(env: &Env, f: F) -> T
where
    F: FnOnce(&Temporary) -> T,
{
    f(&env.storage().temporary())
}

/// Generic proxy for storage operations.
pub struct StorageProxy<'a, K, T>
where
    K: 'a + IntoVal<Env, Val> + TryFromVal<Env, Val>,
{
    key: &'a K,
    _data: PhantomData<*const T>,
}

impl<'a, K, T> StorageProxy<'a, K, T>
where
    K: IntoVal<Env, Val> + TryFromVal<Env, Val>,
{
    fn new(key: &'a K) -> Self {
        StorageProxy {
            key,
            _data: PhantomData,
        }
    }

    pub fn get_key(&self) -> &'a K {
        self.key
    }
}

/// Trait for storage operations.
pub trait StorageOps<T> {
    fn get(&self, env: &Env) -> Option<T>;
    fn set(&self, env: &Env, data: &T);
    fn remove(&self, env: &Env);
    fn has(&self, env: &Env) -> bool;
    fn extend_ttl(&self, env: &Env, threshold: u32, extend_to: u32);
}

pub fn get<'a, K, T>(env: &Env, key: &'a K) -> Option<T>
where
    StorageProxy<'a, K, T>: StorageOps<T>,
    K: IntoVal<Env, Val> + TryFromVal<Env, Val> + ?Sized,
{
    StorageProxy::<'a, K, T>::new(key).get(env)
}

pub fn get_or_else<'a, K, T, F, R>(env: &Env, key: &'a K, handler: F) -> R
where
    StorageProxy<'a, K, T>: StorageOps<T>,
    K: IntoVal<Env, Val> + TryFromVal<Env, Val> + ?Sized,
    F: FnOnce(Option<T>) -> R,
{
    handler(StorageProxy::<'a, K, T>::new(key).get(env))
}

pub fn set<'a, K, T>(env: &Env, key: &'a K, data: &T)
where
    StorageProxy<'a, K, T>: StorageOps<T>,
    K: IntoVal<Env, Val> + TryFromVal<Env, Val> + ?Sized,
{
    StorageProxy::<'a, K, T>::new(key).set(env, data);
}

pub fn has<'a, K, T>(env: &Env, key: &'a K) -> bool
where
    StorageProxy<'a, K, T>: StorageOps<T>,
    K: IntoVal<Env, Val> + TryFromVal<Env, Val> + ?Sized,
{
    StorageProxy::<'a, K, T>::new(key).has(env)
}

pub fn remove<'a, K, T>(env: &Env, key: &'a K)
where
    StorageProxy<'a, K, T>: StorageOps<T>,
    K: IntoVal<Env, Val> + TryFromVal<Env, Val> + ?Sized,
{
    StorageProxy::<'a, K, T>::new(key).remove(env);
}

pub fn extend_ttl<'a, K, T>(
    env: &Env,
    key: &'a K,
    threshold: u32,
    extend_to: u32,
) where
    StorageProxy<'a, K, T>: StorageOps<T>,
    K: IntoVal<Env, Val> + TryFromVal<Env, Val> + ?Sized,
{
    StorageProxy::<'a, K, T>::new(key).extend_ttl(env, threshold, extend_to);
}

#[macro_export]
macro_rules! impl_key_constraint {
    ($key_type:ty, $key_trait:ident) => {
        pub trait $key_trait {}
        impl $key_trait for $key_type {}
    };
}

#[macro_export]
macro_rules! impl_storage {
    (Instance, $data_type:ty $(, $key_trait:ident)?) => {
        impl<'a, K> $crate::storage::StorageOps<$data_type>
            for $crate::storage::StorageProxy<'a, K, $data_type>
        where
            K: $( $key_trait + )? soroban_sdk::IntoVal<soroban_sdk::Env, soroban_sdk::Val>
                + soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>,
        {
            fn get(&self, env: &soroban_sdk::Env) -> Option<$data_type> {
                $crate::storage::with_instance_storage(env, |storage| storage.get(self.get_key()))
            }

            fn set(&self, env: &soroban_sdk::Env, data: &$data_type) {
                $crate::storage::with_instance_storage(env, |storage| {
                    storage.set(self.get_key(), data)
                });
            }

            fn remove(&self, env: &soroban_sdk::Env) {
                $crate::storage::with_instance_storage(env, |storage| {
                    storage.remove(self.get_key())
                });
            }

            fn has(&self, env: &soroban_sdk::Env) -> bool {
                $crate::storage::with_instance_storage(env, |storage| storage.has(self.get_key()))
            }

            fn extend_ttl(&self, env: &Env, threshold: u32, extend_to: u32) {
                $crate::storage::with_instance_storage(env, |storage| {
                    storage.extend_ttl(
                        threshold,
                        extend_to,
                    )
                });
            }
        }
    };
    (Persistent, $data_type:ty $(, $key_type:ident)?) => {
        impl<'a, K> $crate::storage::StorageOps<$data_type>
            for $crate::storage::StorageProxy<'a, K, $data_type>
        where
            K: $( $key_type + )? soroban_sdk::IntoVal<soroban_sdk::Env, soroban_sdk::Val>
                + soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>,
        {
            fn get(&self, env: &soroban_sdk::Env) -> Option<$data_type> {
                $crate::storage::with_persistent_storage(env, |storage| storage.get(self.get_key()))
            }

            fn set(&self, env: &soroban_sdk::Env, data: &$data_type) {
                $crate::storage::with_persistent_storage(env, |storage| {
                    storage.set(self.get_key(), data)
                });
            }

            fn remove(&self, env: &soroban_sdk::Env) {
                $crate::storage::with_persistent_storage(env, |storage| {
                    storage.remove(self.get_key())
                });
            }

            fn has(&self, env: &soroban_sdk::Env) -> bool {
                $crate::storage::with_persistent_storage(env, |storage| storage.has(self.get_key()))
            }

            fn extend_ttl(&self, env: &Env, threshold: u32, extend_to: u32) {
                $crate::storage::with_persistent_storage(env, |storage| {
                    storage.extend_ttl(
                        self.get_key(),
                        threshold,
                        extend_to,
                    )
                });
            }
        }
    };
    (Temporary, $data_type:ty $(, $key_type:ident)?) => {
        impl<'a, K> $crate::storage::StorageOps<$data_type>
            for $crate::storage::StorageProxy<'a, K, $data_type>
        where
            K: $( $key_type + )? soroban_sdk::IntoVal<soroban_sdk::Env, soroban_sdk::Val>
                + soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>,
        {
            fn get(&self, env: &soroban_sdk::Env) -> Option<$data_type> {
                $crate::storage::with_temporary_storage(env, |storage| storage.get(self.get_key()))
            }

            fn set(&self, env: &soroban_sdk::Env, data: &$data_type) {
                $crate::storage::with_temporary_storage(env, |storage| {
                    storage.set(self.get_key(), data)
                });
            }

            fn remove(&self, env: &soroban_sdk::Env) {
                $crate::storage::with_temporary_storage(env, |storage| {
                    storage.remove(self.get_key())
                });
            }

            fn has(&self, env: &soroban_sdk::Env) -> bool {
                $crate::storage::with_temporary_storage(env, |storage| storage.has(self.get_key()))
            }

            fn extend_ttl(&self, env: &Env, threshold: u32, extend_to: u32) {
                $crate::storage::with_temporary_storage(env, |storage| {
                    storage.extend_ttl(
                        self.get_key(),
                        threshold,
                        extend_to,
                    )
                });
            }
        }
    };
}
