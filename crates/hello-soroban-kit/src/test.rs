/*
    Date: 2023
    Author: Fred Kyung-jin Rezeau <fred@litemint.com>
    Copyright (c) 2023 Litemint LLC

    MIT License
*/

extern crate std;
use core::panic::AssertUnwindSafe;
use soroban_macros::{key_constraint, storage};
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, vec, Env, Symbol, Vec};
use soroban_tools::storage;
use std::panic::catch_unwind;

#[contract]
pub struct TestContract;

// Use `key_constraint` to apply a constraint to the Key.
#[contracttype]
#[key_constraint(HelloKeyConstraint)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Key {
    Newcomer,
}

// Use `storage` to implement the desired storage for your
// custom contract type. We also apply the HelloKeyConstraint.
#[contracttype]
#[storage(Instance, HelloKeyConstraint)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Data {
    pub newcomer: Symbol,
}

#[contractimpl]
impl TestContract {
    pub fn hello(env: Env, newcomer: Symbol) -> Vec<Symbol> {
        let key = Key::Newcomer;
        let data = Data { newcomer };

        // Try to get the newcomer from storage, should panic.
        let result = catch_unwind(AssertUnwindSafe(|| {
            assert_eq!(storage::get(&env, &key).unwrap(), data);
        }));
        assert!(result.is_err(), "None set. The operation should panic.");

        // Let's set it then.
        storage::set(&env, &key, &data);
        assert_eq!(storage::has(&env, &key), true);

        // Greetings from storage!
        vec![
            &env,
            symbol_short!("Hello"),
            storage::get(&env, &key).unwrap().newcomer,
        ]
    }
}

#[test]
fn test_hello_soroban_kit() {
    let env = Env::default();
    TestContractClient::new(&env, &env.register_contract(None, TestContract))
        .hello(&symbol_short!("Fred"));
}
