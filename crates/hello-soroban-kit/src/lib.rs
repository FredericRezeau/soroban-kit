/*
    Date: 2023
    Author: Fred Kyung-jin Rezeau <fred@litemint.com>
    Copyright (c) 2023 Litemint LLC

    MIT License
*/

#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, vec, Address, Env, IntoVal, Symbol,
    TryFromVal, Val, Vec,
};

use soroban_macros::{key_constraint, state_machine, storage};
use soroban_tools::{
    fsm::{StateMachine, TransitionHandler},
    storage,
};

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

    pub fn run_state_machine(env: Env, user: Address) {

        // This is just a quick state-machine setup demo

        // For more complete state-machine examples check:
        // - Gaming lobby example: `crates/soroban-macros/tests/state-machine-tests.rs` in soroban-macros integration tests.
        // - Coffee machine example: `crates/hello-soroban-kit/src/test.rs`

        // Declare the state machine.
        struct MyStateMachine;

        // Declare the states
        #[contracttype]
        #[derive(Clone, Debug, Eq, PartialEq)]
        pub enum State {
            Opened,
            Running(Address),
            Closed,
        }

        // Declare the regions.
        #[contracttype]
        #[derive(Clone, Debug, Eq, PartialEq)]
        pub enum Region {
            Global,
            Specific(Address),
        }

        // Implement the TransitionHandler trait for MyStateMachine.
        impl<K, V> TransitionHandler<K, V> for MyStateMachine
        where
            K: IntoVal<Env, Val> + TryFromVal<Env, Val>,
            V: Clone + IntoVal<Env, Val> + TryFromVal<Env, Val> + Into<State>,
        {
            fn on_guard(&self, _env: &Env, _state_machine: &soroban_tools::fsm::StateMachine<K, V>) {}
            fn on_effect(&self, _env: &Env, _state_machine: &soroban_tools::fsm::StateMachine<K, V>) {}
        }

        // Implement MyStateMachine, use the #[state_machine] attribute to
        // declare your state machine functions.
        impl MyStateMachine {
            #[state_machine(
                state = "State:Running:user",
                region = "Region:Specific:user",
                transition = true,
                storage = "temporary"
            )]
            fn run(&self, env: &Env, user: &Address) {
            }

            fn init(&self, env: &Env, user: &Address) {
                let region = Region::Specific(user.clone());
                let state_machine =
                    StateMachine::<Region, State>::new(&region, soroban_tools::fsm::StorageType::Temporary);
                state_machine.set_state(&env, State::Running(user.clone()));
            }
        }

        // Run it.
        let state_machine = MyStateMachine;
        state_machine.init(&env, &user);
        state_machine.run(&env, &user);
    }
}

#[cfg(test)]
mod test;
