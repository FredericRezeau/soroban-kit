/*
    Date: 2023
    Author: Fred Kyung-jin Rezeau <fred@litemint.com>
    Copyright (c) 2023 Litemint LLC

    MIT License
*/

extern crate std;
use core::panic::AssertUnwindSafe;
use std::panic::catch_unwind;
use std::println;

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, testutils::Address as _, vec, Address, Env,
    Symbol, Vec,
};

use soroban_macros::{key_constraint, state_machine, storage};
use soroban_tools::{
    fsm,
    fsm::{StateMachine, TransitionHandler},
    storage,
};

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

// Used to derive extended variables for states.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Item {
    Americano,
    Cappuccino,
}

// Coffee machine states
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum State {
    Refilled,
    Selection,
    Distribution(Item),
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Region {
    All,
    Buyer(Address),
}

pub struct CoffeeMachine;

impl TransitionHandler<Region, State> for CoffeeMachine {
    // Called immediately before state validation.
    // Used to implement guard conditions for the transition.
    fn on_guard(&self, env: &Env, state_machine: &StateMachine<Region, State>) {
        // E.g., Use this handler to implement time based transitions.
        println!(
            "TransitionHandler::on_guard. State = {:?} Region = {:?} Storage = {:?} Timestamp = {}",
            state_machine.get_state(&env).unwrap(),
            state_machine.get_region(),
            state_machine.get_storage_type(),
            env.ledger().timestamp()
        );
    }

    // Called immediately after state validation iff validation succeeded.
    // Used to implement the effect (immediate action) from transitioning.
    // This code block has precedence over the attributed function body.
    fn on_effect(&self, _env: &Env, _state_machine: &StateMachine<Region, State>) {}
}

impl CoffeeMachine {
    // This function panics when vending machine
    // state is not State:Refilled
    #[state_machine(
        state = "State:Refilled",
        region = "Region:All",
        transition = true,
        storage = "temporary"
    )]
    fn insert_coin(&self, env: &Env, account: &Address) {
        self.set_state(&env, State::Selection, &account);
    }

    // This function panics when vending machine
    // state is not State:Selection.
    #[state_machine(
        state = "State:Selection",
        region = "Region:Buyer:account",
        transition = true,
        storage = "temporary"
    )]
    fn select(&self, env: &Env, account: &Address, item: &Item) {
        self.set_state(&env, State::Distribution(item.clone()), &account);
    }

    // This function panics when vending machine
    // state is not State:Distribution(Item), the tuple runtime value must match.
    #[state_machine(
        state = "State:Distribution:item",
        region = "Region:Buyer:account",
        transition = true,
        storage = "temporary"
    )]
    fn distribute(&self, env: &Env, account: &Address, item: &Item) {}

    fn set_state(&self, env: &Env, state: State, account: &Address) {
        let region = Region::Buyer(account.clone());
        let state_machine = StateMachine::<Region, State>::new(&region, fsm::StorageType::Temporary);
        state_machine.set_state(&env, state);
    }

    fn refill(&self, env: &Env) {
        let state_machine =
            StateMachine::<Region, State>::new(&Region::All, fsm::StorageType::Temporary);
        state_machine.set_state(&env, State::Refilled);
    }
}

#[contractimpl]
impl TestContract {
    pub fn hello_state_machine(env: Env) {
        // soroban-kit FSM allows modeling complex concurrency behaviors
        // with regions and extended state variables.

        let buyer1 = Address::random(&env);
        let buyer2 = Address::random(&env);
        let coffee_machine = CoffeeMachine;

        // Set the initial state to Refilled.
        coffee_machine.refill(&env);

        // We can use regions to create a composite state (see #[state_machine] attribute).
        // so buyers can operate the machine concurrently.
        coffee_machine.insert_coin(&env, &buyer1);
        coffee_machine.insert_coin(&env, &buyer2);

        // We can use extended state variables to model the distribution
        // state for each item (Americano and Cappuccino).
        // The state machine transition will panic! if `distribute` is invoked
        // for an item the specific buyer did not select.
        coffee_machine.select(&env, &buyer1, &Item::Americano);
        coffee_machine.select(&env, &buyer2, &Item::Cappuccino);

        // Try to distribute Cappuccino to Buyer1
        let result = catch_unwind(AssertUnwindSafe(|| {
            coffee_machine.distribute(&env, &buyer1, &Item::Cappuccino);
        }));
        assert!(result.is_err(), "The operation should panic. Buyer1 selected Americano");

        coffee_machine.distribute(&env, &buyer1, &Item::Americano);
        coffee_machine.distribute(&env, &buyer2, &Item::Cappuccino);
        
    }

    pub fn hello_storage(env: Env, newcomer: Symbol) -> Vec<Symbol> {
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
fn test_soroban_kit_hello_state_machine() {
    let env = Env::default();
    TestContractClient::new(&env, &env.register_contract(None, TestContract)).hello_state_machine();
}

#[test]
fn test_soroban_kit_hello_storage() {
    let env = Env::default();
    TestContractClient::new(&env, &env.register_contract(None, TestContract))
        .hello_storage(&symbol_short!("Fred"));
}
