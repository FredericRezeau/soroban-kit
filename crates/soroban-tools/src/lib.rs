#![no_std]
/*
    Date: 2023
    Author: Fred Kyung-jin Rezeau <fred@litemint.com>
    Copyright (c) 2023 Litemint LLC

    MIT License
*/

//! A collection of procedural macros designed to streamline development for Soroban
//! smart contracts.
//! 
//! ##State Machine
//! 
//! The `state-machine` attribute macro can be used to implement versatile state machines
//! in Soroban smart contracts. It features state concurrency through regions, runtime behavior
//! modeling via extended state variables, transition control with guards and effects,
//! and state persistence with Soroban storage.
//!
//! ### Background
//!
//! While state machines are a prevalent behavioral pattern in Solidity smart contracts, their
//! implementation is often limited due to Solidity rigid architecture leading to complexities,
//! and sometimes impossibilities, in implementing concurrency and runtime behaviors.
//! 
//! Leveraging Rust advanced type system, soroban-kit `state-machine` can handle more complex interactions
//! and concurrent state executions, enabling a flexible, yet straightforward state machine solution
//! for Soroban smart contracts.
//! 
//! ### Usage
//! 
//! Check out the `game lobby` and `coffee machine` examples for detailed usage:
//! soroban-kit/crates/soroban-macros/tests/state-machine-tests.rs
//! soroban-kit/crates/hello-soroban-kit/src/tests.rs
//!
//! ##Storage
//! 
//! The `storage` and `key_constraint` macros can be used to implement type safety
//! for storage operations while also enforcing type rules at compile time to bind
//! the Soroban storage, data types and keys.
//!
//! For performance, the code generates a minimal wrapper that handles key and data operations
//! without duplication, leveraging Rust lifetimes for safe borrowing.
//!
//! ### Background
//!
//! When dealing with the Soroban storage, repetitive boilerplate code is typically required
//! for encapsulation and type safety over generic storage functions.
//! e.g.
//!  fn set_user_data(key: &userKey, data: &UserData) // Persists user data in storage.
//!  fn get_user_data(key: &userKey) -> UserData // Retrieves user data from storage.
//!
//! These macros streamline this process by automatically generating the boilerplate
//! code, enforcing type rules at compile time, binding the storage with custom data types and
//! optionally, applying Trait constraints to storage keys with `key_constraint`.
//! 
//! ### Usage
//! 
//! Check out the integration tests for detailed usage:
//! soroban-kit/crates/soroban-macros/tests/storage-tests.rs

#[cfg(feature = "state-machine")]
pub mod fsm;

#[cfg(feature = "storage")]
pub mod storage;

#[cfg(any(test, feature = "mock-storage"))]
pub mod test_utils;
