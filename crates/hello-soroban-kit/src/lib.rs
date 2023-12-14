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

use examples::{example_rock_paper_scissors, example_storage};

pub trait HelloContractTrait {
    fn rock_paper_scissors(env: Env);
    fn hello(env: Env, newcomer: Symbol) -> Vec<Symbol>;    
}

#[contract]
pub struct HelloContract;

#[contractimpl]
impl HelloContractTrait for HelloContract {
    // Example implementing "rock paper scissors" with
    // soroban-kit `state-machine, `commit` and `reveal`.
    fn rock_paper_scissors(env: Env) {
        example_rock_paper_scissors::hello(env);
    }

    // Example saying hello with soroban-kit type safe `storage`.
    fn hello(env: Env, newcomer: Symbol) -> Vec<Symbol> {
        example_storage::hello(env, newcomer)
    }
}

#[cfg(test)]
mod test;
