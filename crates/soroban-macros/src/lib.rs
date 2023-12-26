/*
    Copyright (c) 2023-2024 Frederic Kyung-jin Rezeau (오경진 吳景振)

    This file is part of soroban-kit.

    Licensed under the MIT License, this software is provided "AS IS",
    no liability assumed. For details, see the LICENSE file in the
    root directory.

    Author: Fred Kyung-jin Rezeau <fred@litemint.com>
*/

extern crate proc_macro;

#[allow(unused_imports)]
use proc_macro::TokenStream;

/// Oracle macros implementation.
#[cfg(feature = "oracle")]
mod oracle;

/// Commitment scheme macros implementation.
#[cfg(feature = "commitment-scheme")]
mod commit;

/// State machine procedural macros implementation.
#[cfg(feature = "state-machine")]
mod fsm;

/// CircuitBreaker procedural macros implementation.
#[cfg(feature = "circuit-breaker")]
mod circuit_breaker;

/// Storage procedural macros implementation.
#[cfg(feature = "storage")]
mod storage;

#[cfg(feature = "commitment-scheme")]
#[proc_macro_attribute]
pub fn commit(attr: TokenStream, input: TokenStream) -> TokenStream {
    commit::commit(attr, input)
}

#[cfg(feature = "commitment-scheme")]
#[proc_macro_attribute]
pub fn reveal(attr: TokenStream, input: TokenStream) -> TokenStream {
    commit::reveal(attr, input)
}

#[cfg(feature = "state-machine")]
#[proc_macro_attribute]
pub fn state_machine(attr: TokenStream, input: TokenStream) -> TokenStream {
    fsm::state_machine(attr, input)
}

#[cfg(feature = "state-machine")]
#[proc_macro_derive(TransitionHandler)]
pub fn transition_handler_derive(input: TokenStream) -> TokenStream {
    fsm::transition_handler_derive(input)
}

#[cfg(feature = "storage")]
#[proc_macro_attribute]
pub fn storage(attr: TokenStream, input: TokenStream) -> TokenStream {
    storage::storage(attr, input)
}

#[cfg(feature = "storage")]
#[proc_macro_attribute]
pub fn key_constraint(attr: TokenStream, input: TokenStream) -> TokenStream {
    storage::key_constraint(attr, input)
}

#[cfg(feature = "circuit-breaker")]
#[proc_macro_attribute]
pub fn when_opened(attr: TokenStream, input: TokenStream) -> TokenStream {
    circuit_breaker::when(attr, input, true)
}

#[cfg(feature = "circuit-breaker")]
#[proc_macro_attribute]
pub fn when_closed(attr: TokenStream, input: TokenStream) -> TokenStream {
    circuit_breaker::when(attr, input, false)
}

#[cfg(feature = "circuit-breaker")]
#[proc_macro_derive(CircuitBreaker)]
pub fn circuit_breaker_derive(input: TokenStream) -> TokenStream {
    circuit_breaker::derive(input)
}

#[cfg(feature = "oracle")]
#[proc_macro_attribute]
pub fn oracle_subscriber(attr: TokenStream, input: TokenStream) -> TokenStream {
    oracle::oracle_subscriber_attribute(attr, input)
}

#[cfg(feature = "oracle")]
#[proc_macro_attribute]
pub fn oracle_broker(attr: TokenStream, input: TokenStream) -> TokenStream {
    oracle::oracle_broker_attribute(attr, input)
}
