/*
    Date: 2023
    Author: Fred Kyung-jin Rezeau <fred@litemint.com>
    Copyright (c) 2023 Litemint LLC

    MIT License
*/

use soroban_sdk::{Env, IntoVal, TryFromVal, Val};

pub fn with_instance_storage<F, T>(_env: &Env, f: F) -> T
where
    F: FnOnce(&MockStorageInstance) -> T,
{
    f(&MockStorageInstance::new())
}

pub fn with_persistent_storage<F, T>(_env: &Env, f: F) -> T
where
    F: FnOnce(&MockStoragePersistent) -> T,
{
    f(&MockStoragePersistent::new())
}

pub fn with_temporary_storage<F, T>(_env: &Env, f: F) -> T
where
    F: FnOnce(&MockStorageTemporary) -> T,
{
    f(&MockStorageTemporary::new())
}

pub struct MockStorageInstance {}

impl MockStorageInstance {
    pub fn new() -> Self {
        MockStorageInstance {}
    }

    pub fn has<K>(&self, _key: &K) -> bool
    where
        K: IntoVal<Env, Val>,
    {
        true
    }

    pub fn get<K, V>(&self, _key: &K) -> Option<V>
    where
        K: IntoVal<Env, Val>,
        V: TryFromVal<Env, Val>,
    {
        None
    }

    pub fn set<K, V>(&self, _key: &K, _val: &V)
    where
        K: IntoVal<Env, Val>,
        V: IntoVal<Env, Val>,
    {
    }

    pub fn extend_ttl(&self, _threshold: u32, _extend_to: u32) {}

    pub fn remove<K>(&self, _key: &K)
    where
        K: IntoVal<Env, Val>,
    {
    }
}

pub struct MockStoragePersistent {}

impl MockStoragePersistent {
    pub fn new() -> Self {
        MockStoragePersistent {}
    }

    pub fn has<K>(&self, _key: &K) -> bool
    where
        K: IntoVal<Env, Val>,
    {
        true
    }

    pub fn get<K, V>(&self, _key: &K) -> Option<V>
    where
        K: IntoVal<Env, Val>,
        V: TryFromVal<Env, Val>,
    {
        None
    }

    pub fn set<K, V>(&self, _key: &K, _val: &V)
    where
        K: IntoVal<Env, Val>,
        V: IntoVal<Env, Val>,
    {
    }

    pub fn extend_ttl<K>(&self, _key: &K, _threshold: u32, _extend_to: u32)
    where
        K: IntoVal<Env, Val>,
    {
    }

    pub fn remove<K>(&self, _key: &K)
    where
        K: IntoVal<Env, Val>,
    {
    }
}

pub struct MockStorageTemporary {}

impl MockStorageTemporary {
    pub fn new() -> Self {
        MockStorageTemporary {}
    }

    pub fn has<K>(&self, _key: &K) -> bool
    where
        K: IntoVal<Env, Val>,
    {
        true
    }

    pub fn get<K, V>(&self, _key: &K) -> Option<V>
    where
        K: IntoVal<Env, Val>,
        V: TryFromVal<Env, Val>,
    {
        None
    }

    pub fn set<K, V>(&self, _key: &K, _val: &V)
    where
        K: IntoVal<Env, Val>,
        V: IntoVal<Env, Val>,
    {
    }

    pub fn extend_ttl<K>(&self, _key: &K, _threshold: u32, _extend_to: u32)
    where
        K: IntoVal<Env, Val>,
    {
    }

    pub fn remove<K>(&self, _key: &K)
    where
        K: IntoVal<Env, Val>,
    {
    }
}
