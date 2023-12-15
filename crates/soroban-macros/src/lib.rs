/*
    Date: 2023
    Author: Fred Kyung-jin Rezeau <fred@litemint.com>
    Copyright (c) 2023 Litemint LLC

    MIT License
*/

extern crate proc_macro;

#[allow(unused_imports)]
use proc_macro::TokenStream;

/// Commitment scheme macros implementation.
#[cfg(feature = "commitment-scheme")]
mod commit;

/// State machine procedural macros implementation.
#[cfg(feature = "state-machine")]
mod fsm;

/// Storage procedural macros implementation.
#[cfg(feature = "storage")]
mod storage;

#[cfg(feature = "commitment-scheme")]
#[proc_macro_attribute]
pub fn commit(attr: TokenStream, item: TokenStream) -> TokenStream {
    commit::commit(attr, item)
}

#[cfg(feature = "commitment-scheme")]
#[proc_macro_attribute]
pub fn reveal(attr: TokenStream, item: TokenStream) -> TokenStream {
    commit::reveal(attr, item)
}

#[cfg(feature = "state-machine")]
#[proc_macro_attribute]
pub fn state_machine(attr: TokenStream, item: TokenStream) -> TokenStream {
    fsm::state_machine(attr, item)
}

#[cfg(feature = "state-machine")]
#[proc_macro_derive(TransitionHandler)]
pub fn transition_handler_derive(input: TokenStream) -> TokenStream {
    fsm::transition_handler_derive(input)
}

#[cfg(feature = "storage")]
#[proc_macro_attribute]
pub fn storage(attr: TokenStream, item: TokenStream) -> TokenStream {
    storage::storage(attr, item)
}

#[cfg(feature = "storage")]
#[proc_macro_attribute]
pub fn key_constraint(attr: TokenStream, item: TokenStream) -> TokenStream {
    storage::key_constraint(attr, item)
}
