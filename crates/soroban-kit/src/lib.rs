#![no_std]
/*
    Date: 2023
    Author: Fred Kyung-jin Rezeau <fred@litemint.com>
    Copyright (c) 2023 Litemint LLC

    MIT License
*/

//! soroban-kit official repo and documentation:
//! https://github.com/FredericRezeau/soroban-kit

pub use soroban_macros;
pub use soroban_macros::*;

pub use soroban_tools;
pub use soroban_tools::*;

// Explicit since fsm::TransitionHandler trait is feature gated.
#[cfg(feature = "state-machine")]
// Bring this trait in scope for #[derive(TransitionHandler)]
pub use soroban_tools::fsm::TransitionHandler;