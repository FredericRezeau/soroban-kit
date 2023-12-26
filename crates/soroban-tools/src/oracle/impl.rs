/*
    Copyright (c) 2023-2024 Frederic Kyung-jin Rezeau (오경진 吳景振)

    This file is part of soroban-kit.

    Licensed under the MIT License, this software is provided "AS IS",
    no liability assumed. For details, see the LICENSE file in the
    root directory.

    Author: Fred Kyung-jin Rezeau <fred@litemint.com>
*/

use soroban_sdk::{contracttype, Address, Env, IntoVal, TryFromVal, Val, Vec};

// Events interface for Oracle contracts.
pub trait Events<T, V>
where
    T: Clone + IntoVal<Env, Val> + TryFromVal<Env, Val>,
    V: Clone + IntoVal<Env, Val> + TryFromVal<Env, Val>,
{
    fn on_request(_env: &Env, _topic: &T, _envelope: &Envelope) {}
    fn on_sync_receive(_env: &Env, _topic: &T, _envelope: &Envelope, _data: &V) {}
    fn on_async_receive(_env: &Env, _topic: &T, _envelope: &Envelope, _data: &V) {}
    fn on_subscribe(_env: &Env, _topic: &T, _envelope: &Envelope) -> Option<V> {
        None
    }
    fn on_publish(env: &Env, _topic: &T, _data: &V, _publisher: &Address) -> Vec<Envelope> {
        Vec::<Envelope>::new(env)
    }
}

// Envelope for cross-contract calls.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Envelope {
    pub subscriber: Address,
    pub broker: Address,
    pub router: Address,
}
