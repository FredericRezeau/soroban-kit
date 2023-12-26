/*
    Copyright (c) 2023-2024 Frederic Kyung-jin Rezeau (오경진 吳景振)

    This file is part of soroban-kit.

    Licensed under the MIT License, this software is provided "AS IS",
    no liability assumed. For details, see the LICENSE file in the
    root directory.

    Author: Fred Kyung-jin Rezeau <fred@litemint.com>
*/

// This example demonstrates the use of the `circuit-breaker` to implement a pausable activity
// in your smart contract.

// `CircuitBreaker` implements the `TransitionHandler` trait from `state-machine` to manage its
// circuit states, providing similar control over state transitions with `on_guard` and `on_effect`.
// It also supports the creation of composite circuits using `state-machine` regions.

use soroban_sdk::{symbol_short, Env, Symbol};

use soroban_kit::{
    fsm::StateMachine, soroban_tools, when_closed, when_opened, CircuitBreaker, TransitionHandler,
};

use crate::types::Region;

#[derive(CircuitBreaker)]
pub struct PausableActivity;

impl PausableActivity {
    // e.g., use on_guard to implement auth and time based guards
    // on circuit state transitions (opened <-> closed).
    fn on_guard(&self, _env: &Env, _state_machine: &StateMachine<Region, bool>) {
        // admin.require_auth()
    }

    // Can only be called when circuit is opened.
    // +--------/ /--------+
    // |                   |
    // |                   |
    // +-------------------+
    #[when_opened(region = "Region:Circuit")]
    fn can_call_when_paused(&self, env: &Env) {}

    // Can only be called when circuit is closed.
    // +--------/----------+
    // |                   |
    // |                   |
    // +-------------------+
    #[when_closed(region = "Region:Circuit")]
    fn can_call_when_unpaused(&self, env: &Env) {}

    // This trigger closes the circuit when called.
    #[when_opened(region = "Region:Circuit", trigger = true)]
    fn unpause(&self, env: &Env) {}

    // This trigger opens the circuit when called.
    #[when_closed(region = "Region:Circuit", trigger = true)]
    fn pause(&self, env: &Env) {}
}

pub fn hello(env: Env) -> Symbol {
    let pausable_activity = PausableActivity;
    pausable_activity.can_call_when_unpaused(&env);
    pausable_activity.pause(&env);
    pausable_activity.can_call_when_paused(&env);
    pausable_activity.unpause(&env);
    symbol_short!("Success")
}
