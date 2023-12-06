/*
    Date: 2023
    Author: Fred Kyung-jin Rezeau <fred@litemint.com>
    Copyright (c) 2023 Litemint LLC

    MIT License
*/

#![no_std]
use soroban_macros::{key_constraint, storage};
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, vec, Env, Symbol, Vec};
use soroban_tools::storage;

// Optional but recommended.
// Use `key_constraint` to apply a constraint to the Key
// that will be used to operate the storage.
#[contracttype]
#[key_constraint(DataKeyConstraint)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    Newcomer,
}

// Use `storage` to implement the desired storage for your
// custom contract type. We also apply the DataKeyConstraint.
#[contracttype]
#[storage(Instance, DataKeyConstraint)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Data {
    pub newcomer: Symbol,
}

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn hello(env: Env, newcomer: Symbol) -> Vec<Symbol> {
        let key = DataKey::Newcomer;
        let data = Data { newcomer };

        // Let's set newcomer to instance storage.
        storage::set(&env, &key, &data);

        // Greetings from storage!

        // Unlike calling env.storage().instance().get(&key) the compiler can
        // now infer your Option<Data> type as soroban_tools::storage provides
        // a concrete implementation over the Data type.

        // To make sure the Rust type inference engine can always infer
        // types when you use several storage data, you can use key constraints.

        let stored_newcomer = storage::get(&env, &key).unwrap().newcomer;

        vec![&env, symbol_short!("Hello"), stored_newcomer]
    }
}

#[cfg(test)]
mod test;
