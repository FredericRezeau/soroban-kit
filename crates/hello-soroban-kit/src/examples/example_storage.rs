/*
    Date: 2023
    Author: Fred Kyung-jin Rezeau <fred@litemint.com>
    Copyright (c) 2023 Litemint LLC

    MIT License
*/

use soroban_sdk::{symbol_short, vec, Env, Symbol, Vec};

use soroban_kit::storage;

use crate::types::{Data, DataKey};

pub fn hello(env: Env, newcomer: Symbol) -> Vec<Symbol> {
    let key = DataKey::Newcomer;
    let data = Data { newcomer };

    // Set newcomer to instance storage.
    storage::set(&env, &key, &data);

    // Greetings from type safe storage!

    // Unlike calling env.storage().instance().get(&key) the compiler can
    // now infer your Option<Data> type as soroban_kit::storage provides
    // a concrete implementation over the Data type.

    // To make sure the Rust type inference engine can always infer
    // types when you use several storage data, you can use key constraints.

    let stored_newcomer = storage::get(&env, &key).unwrap().newcomer;

    vec![&env, symbol_short!("Hello"), stored_newcomer]
}
