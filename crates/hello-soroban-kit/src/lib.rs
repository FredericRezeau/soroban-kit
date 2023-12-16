/*
    Date: 2023
    Author: Fred Kyung-jin Rezeau <fred@litemint.com>
    Copyright (c) 2023 Litemint LLC

    MIT License
*/

#![no_std]

mod examples; // Examples module.
mod types; // Contract types.

use soroban_sdk::{contract, contractimpl, Env, Symbol, Vec};

use examples::{example_circuit_breaker, example_rock_paper_scissors, example_storage};

pub trait HelloContractTrait {
    fn circuit_breaker(env: Env) -> Symbol;
    fn rock_paper_scissors(env: Env) -> Symbol;
    fn hello(env: Env, newcomer: Symbol) -> Vec<Symbol>;
}

#[contract]
pub struct HelloContract;

#[contractimpl]
impl HelloContractTrait for HelloContract {
    // Example implementing a pausable activity with
    // soroban-kit `circuit-breaker`, `when_opened` and `when_closed` attr. macros.
    fn circuit_breaker(env: Env) -> Symbol {
        example_circuit_breaker::hello(env)
    }

    // Example implementing "rock paper scissors" with
    // soroban-kit `state-machine`, `commit` and `reveal` attr. macros.
    fn rock_paper_scissors(env: Env) -> Symbol {
        example_rock_paper_scissors::hello(env)
    }

    // Example saying hello with soroban-kit type safe `storage` attr. macros.
    fn hello(env: Env, newcomer: Symbol) -> Vec<Symbol> {
        example_storage::hello(env, newcomer)
    }
}

#[cfg(test)]
mod test;
